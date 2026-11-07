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
    front_path = cfg['selected']
    for key in ('currentStage3FrontendValidation', 'integratedStage3FrontendValidation', 'schema9Stage3FrontendValidation'):
        candidate_front = read(cfg.get(key))
        if candidate_front.get('compiler_source_sha256') and candidate_front.get('rows'):
            front_path = cfg[key]
    front = read(front_path)
    scope = read(cfg['coverage'])
    outside_review = read(root/'bootstrap/current-outside-frontend-target-review.json')
    chain = read(Path(cfg['chain'])/'independent-verification.json')
    newer_chain = read(root/'bootstrap/current-partial-boolean-chain-results.json')
    if chain.get('compiler_source_sha256') != compiler and newer_chain.get('status') == 'passed-bootstrap-chain-only':
        chain = newer_chain
    integrated_proof = read(cfg.get('integratedTaskLawChainReceipt'))
    if chain.get('compiler_source_sha256') != compiler and integrated_proof.get('status') == 'passed-bootstrap-chain-only':
        chain = integrated_proof
    pinned_audit = read(root/'bootstrap/current-schema9-bootstrap-results.json')
    if pinned_audit.get('status') == 'passed-production-schema9-bootstrap-chain':
        chain = pinned_audit
    schema10_audit_path = 'bootstrap/current-schema10-bootstrap-results.json'
    schema10_audit = read(root/schema10_audit_path)
    strict_audit_path = 'bootstrap/current-schema10-strict-bootstrap-results.json'
    strict_audit = read(root/strict_audit_path)
    if strict_audit.get('compiler_source_sha256') == compiler and strict_audit.get('status') == 'passed-production-schema10-bootstrap-chain':
        chain = strict_audit
    elif schema10_audit.get('status') == 'passed-production-schema10-bootstrap-chain':
        chain = schema10_audit
    earlier_full = read(cfg.get('previousNative')) or full
    earlier_native = {r['name']: r for r in earlier_full.get('rows', [])}
    old_native = {r['name']: r for r in full.get('rows', [])}
    old_front = {r['name']: r for r in front.get('rows', [])}
    catalog = read(Path(cfg['native']).parent/'test-binaries.json')
    native_names = sorted(set(catalog) | set(old_native) | {'doc'} | {p.stem for p in (root/'compiler/tests').glob('*.rs')})
    front_names = scope['portable_targets'] + scope['interface_api_targets']
    assert len(native_names) >= 235
    assert len(front_names) == len(set(front_names)) >= 105
    assert set(front_names) >= set(scope['original_targets'])
    assert set(front_names) <= set(native_names)

    # Credit current Rust-only groups only when their production identity and
    # their exact test source match the current published inputs. These receipts
    # do not satisfy the separate final-Stage3 frontend requirement.
    current_native = {}
    for key in ('nativeQuotedReceipt', 'nativeDefaultAnnotationReceipt', 'nativePipelineReceipt', 'nativeLiteralContextReceipt', 'nativeIndexKindReceipt', 'nativeArrayFixtureReceipt'):
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
            if name.startswith('bootstrap_') or (name != 'lib' and '../../bootstrap/' in (source/relative).read_text()):
                if receipt.get('compiler_source_sha256') != compiler:
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
            observed=observed(old, full.get('compiler_source_sha256'), Path(cfg['native']).parent) if old and old.get('status') != 'pending' else observed(earlier_native.get(name), earlier_full.get('compiler_source_sha256'), Path(cfg.get('previousNative',cfg['native'])).parent),
            currentTests=current.get('tests') if current else None))

    front_rows = []
    for name in front_names:
        old = old_front.get(name)
        same = (front.get('compiler_source_sha256') == compiler
                and front.get('native_source_sha256') == native)
        state = {'passed':'verified', 'failed':'blocked', 'running':'running'}.get(old.get('status'), 'pending') if same and old else 'recheck' if old and old.get('status') != 'pending' else 'pending'
        front_rows.append(item('stage3:'+name, name, state,
            'Requires the latest genuinely self-built Stage3. Development-compiler checks are listed separately.',
            'Run the complete original target through the matching Stage3; separately classify direct Rust API assertions.',
            link('compiler/tests/'+name+'.rs'), observed=observed(old, front.get('compiler_source_sha256'), Path(front_path).parent),
            category='interface / API' if name in scope['interface_api_targets'] else 'portable behavior'))

    suites = [suite('native', 'Rust compiler suite', 'targets', native_rows,
        '235 preserved targets, including library, integration, binary and documentation targets. A target can contain many tests.',
        observedCounts=dict(collections.Counter(r.get('status') for r in earlier_full.get('rows', []))), observedSource=earlier_full.get('compiler_source_sha256')),
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
    latest_aux = read(cfg.get('currentAuxiliaryReceipt'))
    latest_aux_matches = (latest_aux.get('status') == 'audited-all-current-native-tooling-and-editor-tests'
                          and latest_aux.get('compiler_source_sha256') == compiler
                          and latest_aux.get('native_source_sha256') == native
                          and latest_aux.get('tooling_tests') == 43 and latest_aux.get('editor_tests') == 39)
    for id, title, log_name, total_key, required in [
        ('runtime','Runtime suite','runtime.log','runtime_tests',171),
        ('tooling','Bootstrap and tooling suite','tooling.log','tooling_tests',43),
        ('editor','Syntax and editor suite','syntax-editor.log','editor_tests',39)]:
        found = cases(aux_root/log_name)
        assert len(found) == auxiliary.get(total_key) == required, (id, len(found), auxiliary.get(total_key))
        same = runtime_current if id == 'runtime' else auxiliary.get('compiler_source_sha256') == compiler and auxiliary.get('native_source_sha256') == native
        current_run = next((r for r in latest_aux.get('runs', []) if r['name'] == log_name.removesuffix('.log')), None) if latest_aux_matches and id != 'runtime' else None
        if current_run:
            assert current_run['tests'] == required and {r['name'] for r in current_run['cases']} == {r['name'] for r in found}
            found = current_run['cases']
            same = current_run['status'] == 'passed'
        rows = [item(id+':'+c['name'], c['name'], 'verified' if same and c['status']=='passed' else 'blocked' if same and c['status']=='failed' else 'recheck',
            'Passed; complete runtime source and test inputs are unchanged.' if id=='runtime' and same else 'Passed the complete current tooling/editor suite with audited Native8ba; final Cinch-built language coverage remains separate.' if current_run and same else 'Last run belongs to an earlier compiler snapshot.',
            'The complete original test passes against matching code and its required compiler role.',
            link('bootstrap/current-native-tooling-editor-results.json') if current_run else link('bootstrap/current-record-indexed-auxiliary-results.json'),
            observed=dict(status=c['status'], source=latest_aux.get('compiler_source_sha256') if current_run else auxiliary.get('compiler_source_sha256'), cases=[])) for c in found]
        suites.append(suite(id, title, 'tests', rows,
            'Exact runtime/test input equivalence checked against the retained successful run.' if id=='runtime' and same else 'All original tests pass on current published inputs with audited Native8ba. Native editor integration stays separate from final self-built Cinch frontend acceptance.' if current_run and same else 'Older passing results remain visible; matching compiler/editor frontend execution is still required.'))

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
    language_evidence = Path(cfg['currentExternalBehaviorReceipt']).name
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
                link('bootstrap/'+language_evidence),
                observed=dict(status=status,source=language.get('compiler_source_sha256'),cases=[])))
    suites.append(suite('language','Language behavior and refusal reasons','cases',lang_rows,'All 13 valid examples and all 21 intended-reason refusal checks.'))

    additional = read(cfg.get('sourceClassDefaultApiReceipt'))
    if additional:
        group = next(r for r in additional['groups'] if r['name'] == 'class_defaults')
        row = item('additional:class_defaults', 'class_defaults / actual Cinch frontend', 'recheck',
            'All seven original/new default tests remain. Private75fb passes3/7, improved from1/7: both original strict-value runtime/build tests pass. Three exact diagnostic/source assertions and the refinement client remain failures.',
            'Pass every original default source, runtime, identity and diagnostic/path assertion through the final matching Cinch-built compiler; classify the11 direct Rust annotation checks separately.',
            link('bootstrap/current-source-class-default-self-values-api-results.json'),
            observed=dict(status='failed',source=additional['compiler_source_sha256'],cases=cases(Path(cfg.get('artifactRoot', BASE))/'source-class-default-interface-api-2/class_defaults.log')))
        extra_rows = [row]
        concurrency = read(cfg.get('originalConcurrencyReceipt'))
        if concurrency:
            extra_rows.append(item('additional:concurrency', 'concurrency', 'blocked',
                f"Development Cinch passed {concurrency['passed_tests']} of {concurrency['tests']} original concurrency tests. The failed cases retain their first blocker.",
                'Run the complete original target through matching Stage3, including original runtime, I/O, foreign fixture and exact refusal reasons.',
                link('bootstrap/current-original-concurrency-frontend-results.json'),
                observed=dict(status='failed',source=concurrency['compiler_source_sha256'],
                    cases=[dict(name=case['name'], status=case['status']) for case in concurrency['cases']])))
        for reviewed in outside_review.get('rows', []):
            name = reviewed['target']
            if name in ('laws', 'prim'):
                extra_rows.append(item('additional:'+name, name, 'pending',
                    f"The Rust target passed all {reviewed['rust_test_function_count']} tests. Matching Stage3 source coverage is pending. "+reviewed['findings'],
                    reviewed['required'], link('bootstrap/current-outside-frontend-target-review.json')))
        suites.append(suite('additional','Additional frontend coverage','targets',extra_rows,
            'The coverage audit adds these targets alongside all original Stage3 targets. Direct Rust calls have their own test inventory.'))

    acceptance_text = [
        ('Pinned seed builds Stage1', 'The pinned Rust seed builds all latest compiler sources; Stage1 is runnable and passes its executable smoke checks.'),
        ('Stage1 builds Stage2', 'Stage1 freshly rebuilds every compiler module from those same inputs, without a Rust frontend or prebuilt compiler substitution.'),
        ('Stage2 builds matching Stage3', 'Build and run Stage3; Stage2/Stage3 emitted IR, executable and every canonical interface agree.'),
        ('Changed-source bootstrap works', 'Change actual compiler source, rebuild it through the chain, and prove that its changed behavior reaches the consumer.'),
        ('All tests and frontend parity pass', 'Complete all preserved compiler, runtime, tooling, editor and external behavior coverage, including ignored cases and the final coverage audit.')]
    chain_matches = chain.get('compiler_source_sha256')==compiler
    acceptance=[]
    for i,(title,requirement)in enumerate(acceptance_text,1):
        proven = i < 5 and chain.get('status') in ('passed-bootstrap-chain-only', 'passed-production-schema9-bootstrap-chain',
                                                  'passed-production-schema10-bootstrap-chain')
        state = 'verified' if chain_matches and proven else 'recheck' if proven else 'pending'
        if state == 'verified':
            detail = 'The independent audit passed for the published compiler.'
        elif state == 'recheck':
            detail = 'The audit passed for earlier compiler ' + chain.get('compiler_source_sha256','unknown')[:8] + '.'
        else:
            detail = 'Complete matching verification is pending.'
        receipt = 'design/self-hosting.md#acceptance'
        if proven:
            if chain is strict_audit:
                receipt = strict_audit_path
            elif chain is schema10_audit:
                receipt = schema10_audit_path
            elif chain is pinned_audit:
                receipt = 'bootstrap/current-schema9-bootstrap-results.json'
        acceptance.append(item('acceptance:'+str(i),str(i)+'. '+title,state,detail,
            requirement,link(receipt), observed=dict(status='passed'if proven else'pending',source=chain.get('compiler_source_sha256'),cases=[])))

    api=read(cfg.get('interfaceApiReceipt'))
    # A preserved failure from the unchanged actual Cinch frontend remains a
    # blocker while its Rust public-reader replay is pending. This grants no
    # current positive API credit to a different reader.
    api_current=api.get('compiler_source_sha256')==compiler
    failed_api=set(api.get('failed_groups',[])) if api_current else set()
    annotation_receipt=read(cfg.get('nativeDefaultAnnotationReceipt'))
    annotation_published=annotation_receipt.get('native_source_sha256')==native and annotation_receipt.get('whole_groups')==21 and annotation_receipt.get('rust_tests')==332
    default_receipt=read(cfg.get('sourceClassDefaultComponentReceipt'))
    default_runtime=read(cfg.get('sourceClassDefaultRuntime'))
    strict_runtime=read(cfg.get('sourceClassDefaultSelfValuesRuntimeReceipt'))
    default_api=read(cfg.get('sourceClassDefaultApi'))
    default_full=read(cfg.get('sourceClassDefaultFullCompiler'))
    coverage_state=read(cfg.get('inferredCaseCoverageValidation'))
    coverage_runtime=read(cfg.get('inferredCaseCoverageRuntimeReceipt'))
    function_defaults=read(cfg.get('functionDefaultApiReceipt'))
    source_overlay=read(cfg.get('sourceInterfaceOverlayReceipt'))
    early_base=read(cfg.get('sourceOverlayBaseScopeReceipt'))
    cinch_forwarder=read(root/'bootstrap/current-cinch-task-source-forwarder-results.json')
    native_forwarder=read(root/'bootstrap/current-native-task-source-forwarder-results.json')
    stage3_failure_path='bootstrap/current-stage3-arrays-and-library-overrides-results.json'
    stage3_failures=read(root/stage3_failure_path)
    latest_chain=read(cfg.get('partialBooleanBootstrapChain'))
    stage3_failures_current=bool(stage3_failures) and stage3_failures.get('compiler_source_sha256') == latest_chain.get('compiler_source_sha256')
    overlay_result = old_front.get('interface_prelude_overrides', {})
    integrated_overlay = (integrated_proof.get('compiler_source_sha256') == front.get('compiler_source_sha256')
                          and overlay_result.get('status') == 'failed' and overlay_result.get('log')
                          and Path(overlay_result['log']).is_file()
                          and 'does not define method mconcat' in Path(overlay_result['log']).read_text())
    default_detail='Latest75fb passes all seven complete default/dependent component groups (nine tests), all nine full compiler/object tests and both original strict-value runtime/build tests. Original API/default replay:16/21 tests pass; source-Prelude, three diagnostic assertions and the refinement runtime client remain failures. Imported checked default transport and matching final bootstrap remain required.'
    projection = read(cfg.get('structuralProjectionCases'))
    projection_proof = read(cfg.get('structuralProjectionReceipt'))
    copyout = read(cfg.get('copyoutValidation'))
    copyout_proof = read(root/'bootstrap/current-copy-out-development-results.json')
    laws = read(cfg.get('classLawObservationValidation'))
    arrow_receipt = read(cfg.get('arrowFormerReceipt'))
    law_receipt = read(cfg.get('classLawObservationReceipt'))
    default_transport = read(cfg.get('portableDefaultValidation'))
    schema9_validation = read(cfg.get('unifiedDefaultValidation'))
    schema9_native = read(cfg.get('unifiedDefaultNative'))
    schema9_provider = read(cfg.get('unifiedDefaultProvider'))
    schema9_chain = read(cfg.get('unifiedDefaultChain'))
    schema9_cross = read(root/'bootstrap/current-schema9-default-replay-results.json')
    family_audit_path = 'bootstrap/current-schema10-family-and-cutover-results.json'
    family_audit = read(root/family_audit_path)
    cutover = family_audit.get('format_cutover', {})
    unified_format = (cutover.get('status') == 'passed'
                      and family_audit.get('compiler_source_sha256') == compiler)
    stage_syntax = read(root/'bootstrap/current-staging-surface-results.json')
    class_interface = read(cfg.get('classInterfaceParity'))
    class_interface_proof = read(root/'bootstrap/current-class-interface-exports-results.json')
    class_arrow_native = read(cfg.get('classArrowNative'))
    class_arrow_proof = read(root/'bootstrap/current-native-arrow-contract-results.json')
    schema9_original_overlays = read(cfg.get('schema9OriginalOverlays'))
    schema9_pinned_provider = read(cfg.get('schema9PinnedProvider'))
    schema9_pinned_chain = read(cfg.get('schema9PinnedChain'))
    schema9_pinned_proof = read(root/'bootstrap/current-schema9-pinned-provider-results.json')
    schema9_ordering_contracts = read(cfg.get('schema9OrderingContracts'))
    stage_evaluation = read(cfg.get('stageEvaluation'))
    stage_evaluation_proof = read(root/'bootstrap/current-staging-evaluation-results.json')
    stage_expansion_proof = read(root/'bootstrap/current-staging-expansion-results.json')
    combined_arrays = read(root/'bootstrap/current-stage1-combined-arrays-results.json')
    allocation_replay = read(root/'bootstrap/current-stage3-allocation-object-replay-results.json')
    stage_expansion = read(cfg.get('stageExpansion'))
    nominal_dependency = read(cfg.get('schema9NominalDependencies'))
    instance_header = read(cfg.get('schema9InstanceHeader'))
    finite_law = read(cfg.get('finiteLaw'))
    finite_law_pipeline = read(root/'bootstrap/current-schema9-finite-law-pipeline-results.json')
    typed_law = read(cfg.get('typedLawSampling'))
    native_typed_law = read(cfg.get('nativeTypedLawSampling'))
    native_typed_law_proof = read(root/'bootstrap/current-native-mathematical-law-sampling-results.json')
    native_numeric_method_proof = read(root/'bootstrap/current-native-numeric-and-class-method-contract-results.json')
    schema10_integrated_proof = read(root/'bootstrap/current-schema10-stage1-and-integrated-input-results.json')
    portable_law = read(cfg.get('portableLawCinch'))
    family_replay = read(cfg.get('arrowFamilyReplay'))
    gaps=[
        item('gap:prelude','Replace source copies of library modules safely','blocked' if integrated_overlay or stage3_failures_current or 'interface_prelude_overrides' in failed_api else 'pending',
             'The Ordering comparison found differences in instance headers, the direct Data.Core dependency and law observations. The focused nominal dependency regression passed. Matching the complete original library chain remains open.' if any(row.get('name') == 'bootstrap_source_instances' and row.get('status') == 'passed' for row in nominal_dependency.get('rows', [])) else
             'The original test passed Core and rechecked Ordering with the inherited default. Relation.Ord still records a different Ordering interface hash. An exact source/interface comparison is in progress.' if schema9_pinned_proof.get('original_source_family_replay') else
             'The Core family passed. Schema 9 transports the inherited Ordering default. The rebuilt Rust provider verifies all 55 interfaces and preserves both Category arrow instances. The exact four-family test is queued with this provider. Display scheme parity remains open.' if class_arrow_proof else
             'The Core family passed. Schema 9 transports the inherited Ordering default. The Cinch writer now matches Rust on class type and layout rows and keeps instance factories in the instance table. The Category instance head and display scheme still need matching output.' if class_interface_proof else
             'The integrated Stage3 passed the original Core family and reached Ordering, where the inherited mconcat implementation is unavailable. Category and Algebra still need matching results.' if integrated_overlay else
             'The earlier Stage3 refuses the original Data.Core source as a conflicting loaded interface. Development Data.Core emits the matching contract. Ordering, Category and Algebra still need their original replacement checks.' if stage3_failures_current else 'Data.Core emits the matching contract. Data.Ordering stops at the imported mconcat default. Category rejects the class laws and structural function instance. Algebra now resolves Nil, None, Cons and Some and stops at the class law.' if early_base else 'The development pipeline checks each original source family separately. Data.Core emits the matching contract. Data.Ordering stops at the imported mconcat default. Category rejects the class laws and structural function instance. Algebra rejects the class law and cannot resolve Nil, None, Cons or Some.' if source_overlay else 'The current interface/API run fails when source Data.Core is supplied alongside its cached interface. All four original families remain required.',
             'Core, Ordering, Category and Algebra replacements pass with fresh checked provider hashes, valid consumer chains and no Prelude back-edge.',link('bootstrap/current-schema9-pinned-provider-results.json' if schema9_pinned_proof.get('original_source_family_replay') else 'bootstrap/current-native-arrow-contract-results.json' if class_arrow_proof else 'bootstrap/current-class-interface-exports-results.json' if class_interface_proof else stage3_failure_path if stage3_failures_current else 'bootstrap/current-source-overlay-family-probes.json' if source_overlay else 'bootstrap/current-source-prelude-counterexample.json')),
        item('gap:source-defaults','Check and inherit source class defaults','running',default_detail if default_receipt else 'Development compiler8fce passes all15 controls in a retained-checker replay with the corrected Rust oracle; complete groups and runtime integration remain separate.',
             'All source default controls, unused errors, generic scopes, metadata stability, runtime and original class-default behavior pass.',link('bootstrap/current-source-class-default-development.json')),
        item('gap:type-term-scope','Keep term and type annotation names separate','running',
             'Private8fce rejects identity a = (a :: a). Candidate00b5 separates proper type-head lookup from dependent value-index lookup. Both00b5 and75fb pass independent audit of all seven complete default/dependent groups (nine tests). Actual75fb executes the unchanged annotation client42. Published3268 still requires promotion and matching bootstrap.',
             'The unchanged source executes42 through the repaired actual compiler, all16 source default controls pass, and dependent indices retain their checked identities.',link('bootstrap/current-source-class-default-runtime-results.json')),
        item('gap:strict-defaults','Initialize nullary default fields from checked dependencies','running' if strict_runtime else 'blocked',
             'Candidate28d2 executes all11 positive clients, including the unchanged owned Box case, with zero leaks; all5 invalid-source controls refuse. Strict cycles refuse with empty IR. Independent interface-only and public-reader checks prove the fresh checked sidecars are valid; the invented no-sidecar-after-backend-refusal assertion was a runner mistake. Complete matching bootstrap and promotion remain.' if strict_runtime else 'Private8fce checks duplicate = value, then cannot lower its strict self-evidence dependency. Two original default runtime/build assertions also stop here.',
             'Initialize acyclic strict fields in dependency order while preserving checked self evidence, closure ownership and cycle refusals; execute all original default runtime cases with zero leaks.',link('bootstrap/current-source-class-default-runtime-results.json')),
        item('gap:default-diagnostics','Preserve original class/default diagnostic details and source files','running',
             'All three original name/source-file assertions now pass in the audited14a8 replay: defaults6/7 and combined API/default19/21. The unchanged source-library replacement and mconcat refinement program still fail. Complete matching self-built verification and promotion remain; the earlier4f source-admission failures are retained separately.',
             'Every original diagnostic and filename assertion passes with accurate attribution to the defining source; semantic rejection alone grants no pass.',link('bootstrap/current-source-origin-api-results.json')),
        item('gap:boolean-implications','Retain valid facts from either short-circuit operand','running',
             'A true opaque && ordered condition must retain ordered; a false opaque || unordered condition must retain not unordered. Core currently retains only a readable test operand when the entire condition is opaque. Candidate32ab combines sound selected-branch implications without changing exact condition readings. The first new fixtures used reserved opaque as a parameter; their parser failures are preserved and grant no semantic rejection credit. All nine corrected flag fixtures pass independent Native audit: five valid checks and four actual ordering/precondition refusals. The complete original Cinch precondition target passes with all nine controls and all original assertions retained; all44 modules pass Native admission. Eight further whole groups and matching own-source/bootstrap checks remain.',
             'Run the original precondition, Boolean, Core and refinement groups, preserve all negative controls, and prove the repaired compiler reads its own44 source modules and freshly bootstraps.',link('bootstrap/current-partial-boolean-branch-development-2.json')),
        item('gap:inferred-case-coverage','Infer a constructor case before deciding coverage','running' if coverage_state else 'blocked',
             'Private28d2 now compiles the exact unchanged Box counterexample and executes7 at32M with poisoning and zero leaks. All9 full compiler/object tests and both exact44 source gates pass independent audit. All eight complete component groups (ten Rust tests) pass independent audit; all original tests and assertions are retained alongside eight Native-compared controls. Published3268 still awaits validated promotion and a matching bootstrap.',
             'Infer the checked constructor family without borrowing branch-local index equalities as global coverage proof; retain every existing indexed/GADT refusal and run the owned constructor client with zero leaks.',link('bootstrap/current-inferred-case-coverage-runtime-results.json')),
        item('gap:default-refinement','Preserve refinement carriers in the original default runtime program','running' if read(cfg.get('refinementContainerValidation')) else 'blocked',
             'With the two test-provider factories, the development compiler passes all seven original class-default tests. The original mconcat program also passes at 32 MiB with poisoning and zero leaks. Replay through final Stage3 remains.' if function_defaults.get('default_tests_passed') == 7 else 'The development compiler checks the original mconcat program. The replay needs a test library object with both checked function dictionary factories.',
             'The original function carrier, refined product and unit cases compile and execute their expected results through final Stage3.',link('bootstrap/current-function-default-api-results.json' if function_defaults else 'bootstrap/current-source-class-default-self-values-api-results.json')),
        item('gap:imported-defaults','Use one interface format for Rust and Cinch',
             'verified' if unified_format else 'running' if cfg.get('portableDefaultWork') else 'pending',
             'Rust, Cinch and the verifier require schema 10. A real schema 9 file and fingerprint-only defaults were refused. The pinned producer rebuilt all 55 library interfaces.' if unified_format else
             'The combined schema 9 candidate passed admission of all 44 compiler modules. Its full class-default test inherited checked Rust and Cinch defaults and rejected both tampered and digest-only artifacts. The staged bootstrap and remaining component groups are running.' if schema9_cross.get('combined_candidate') else
             'The schema 9 candidate uses one checked clause array per default. All 15 Rust default and interface tests passed. A Cinch-built consumer inherited both producers’ defaults, and the rebuilt provider verified all 55 library interfaces. The combined candidate is replaying the full component groups and preparing a fresh bootstrap.' if schema9_cross.get('status') == 'passed' else
             'The schema 9 candidate carries one checked clause array in each default. All 15 Rust default and interface tests passed, including the real mconcat body. The Cinch inheritance replay and library rebuild are tracked separately.' if schema9_native.get('status') == 'passed' else
             'The original Ordering bundle needs the inherited Monoid mconcat implementation. The schema 9 candidate carries the defining clauses and the importing compiler rechecks them.',
             'Rebuild library interfaces in the same format both compilers use and verify that obsolete formats are refused.',link(family_audit_path if unified_format else 'bootstrap/current-schema9-default-replay-results.json' if schema9_cross else 'design/compiled-interface-metadata.md')),
        item('gap:laws','Elaborate source class laws and declarations','running',
             'The separate imported-law replay reached the checker and exposed an incorrect terminator on a named law binder. The corrected candidate parses and is queued with Nat and generic constructor-field counterexamples.' if typed_law else
             'The pipeline and interface-writer groups passed audit. A lawful Boolean implementation reports two checked samples, an unlawful one refuses by name, and the unsampled function carrier is explicitly reported as unchecked. General sampling and imported law declarations remain open.' if law_receipt else
             'The checker verifies one carrier for both law operands and refuses incompatible types. The Boolean sampler follows the actual checked instance fields.',
             'Retain and check the original class laws and namespace declarations; pass all original family and law assertions.',link('bootstrap/current-class-law-observations-results.json' if law_receipt else 'bootstrap/current-source-class-law-instance-samples-results.json')),
        item('gap:base-names','Restore source-family implicit base names','running' if early_base else 'pending',
             'The development compiler now resolves the original Algebra.Semigroup constructors. Explicit and implicit Data.Core usage give the same checked contract and exact dependency hash; qualified access still requires an import.' if early_base else 'With the complete authenticated library interface set, source Algebra.Semigroup still cannot resolve Nil, None, Cons and Some before Prelude.',
             'Source library homes retain their checked base scope without a facade back-edge or unchecked aliases.',link('bootstrap/current-source-overlay-base-scope-results.json' if early_base else 'bootstrap/current-source-prelude-counterexample.json')),
        item('gap:refusal-diagnostics','Replay record and index diagnostics on Stage3','recheck',
             'The four original Query, Audio, Circuit and TensorADShape refusals now match the expected reason and source location in a development build. The pipeline and typed-index tests pass.',
             'Integrate the repair and replay all four unchanged cases through the matching Stage3.',link('bootstrap/current-external-refusal-diagnostics-results.json')),
        item('gap:task-callback','Reject mutable task captures hidden by callbacks','blocked',
             'A Rust development patch rejects the original unsafe local callback, keeps safe aliases accepted, and passes all concurrency and unit tests. The published compiler still accepts the counterexample.',
             'The callback carries checked capture, transfer and lifetime facts. The published Rust and Cinch compilers refuse the unsafe capture and accept safe callbacks.',link('bootstrap/current-native-local-task-capture-results.json')),
        item('gap:task-helper','Reject mutable task captures hidden by helpers','blocked','Cinch development passes six controls including both unchanged Tasks examples and the unsafe Bytes refusal. Rust development passes the original helper regression, 19 concurrency tests and 186 unit tests. Portable transfer and replay on the matching compilers remain.',
             'Capture requirements survive helpers, aliases and interfaces, with both bad and valid cases checked.',link('bootstrap/current-cinch-task-source-forwarder-results.json' if cinch_forwarder else 'bootstrap/current-native-task-capture-counterexample.json')),
        item('gap:rust-annotations','Preserve Rust class/default annotation binders','verified' if annotation_published else 'blocked','Published8ba8 passes independent audit: all21 complete groups,332 tests, both44-module projects, actual inherited execution42 and three old/new CLI comparisons. Original refusal controls remain.' if annotation_published else 'The published995f compiler rejects three valid annotation examples. The private8ba8 repair has finished execution of its affected groups and both44-module checks; independent audit and publication remain required.',
             'Complete the affected proof/syntax/interface groups and both compiler-source projects, retain kind/scope refusals, then publish the validated repair.',link('bootstrap/current-native-default-annotation-results.json') if annotation_published else link('design/next-steps.md')),
        item('gap:records','Finish record interface transport','recheck','A known earlier compiler gap; latest exact behavior still needs a complete check.',
             'The original record construction, projection, privacy, layout and interface assertions pass on the matching compiler.',link('design/next-steps.md')),
        item('gap:copyout','Finish Copy / borrow / Thunk ownership','recheck' if copyout_proof else 'blocked' if stage3_failures_current else 'recheck',
             'The development pipeline and runtime groups pass. A shared immutable array is accepted, affine Thunk elements refuse for Copy, and the tuple case returns 42 with zero leaks. The combined compiler still needs its own Stage3 replay.' if copyout_proof else
             'The earlier Stage3 fails to resolve copyOut before checking Copy evidence for Array (Thunk {} Int).',
             'The original ownership case reaches and passes its required semantic checks and runtime behavior.',link('bootstrap/current-copy-out-development-results.json' if copyout_proof else stage3_failure_path)),
        item('gap:staging','Finish staging and namespace behavior','running' if stage_syntax else 'blocked' if stage3_failures_current else 'recheck',
             'The development compiler generated, linked and ran a typed Nat program with zero leaks. It also refused the unchanged original array case by primitive name and original location. The full staging pipeline and matching arrays group remain open.' if stage_expansion_proof else
             'The checked evaluator computes a stage function, refuses arrayReplicate by its resolved identity and enforces a shared work budget. The full pipeline and original arrays group still need their matching results.' if stage_evaluation_proof else
             'The development parser passes the complete syntax target with checked staging nodes and splices. The evaluator still needs to issue the named arrayReplicate refusal and expand valid generators.' if stage_syntax else 'Exact candidate Stage3 reports an unsupported declaration before checking whether arrayReplicate is available at compile time.' if stage3_failures_current else 'The original array and language cases require staging and declaration behavior.',
             'Preserve original sources and expected named refusals; execute the matching staging and language suites.',link('bootstrap/current-staging-expansion-results.json' if stage_expansion_proof else 'bootstrap/current-staging-evaluation-results.json' if stage_evaluation_proof else 'bootstrap/current-staging-surface-results.json' if stage_syntax else stage3_failure_path if stage3_failures_current else 'bootstrap/current-original-array-suite-results.json')),
        item('gap:hidden-types','Stop choosing a concrete hidden constructor type','pending','Native existential/hidden-type introduction remains a required investigation and repair.',
             'Hidden types remain existential, with correct construction, scoped elimination, kind checks and no guessed Nat carrier.',link('design/next-steps.md')),
        item('gap:fixities','Reverify defining-module-only fixities','recheck','The earlier self-built controls prove that both identical and changed importer declarations refuse.',
             'Repeat the original defining-owner, facade/rename and importer-refusal controls through the final matching Stage3.',link('bootstrap/current-record-indexed-external-acceptance-results.json')),
        item('gap:coverage','Audit every frontend-facing test outside the Stage3 matrix','pending',
             f"{len(set(native_names)-set(front_names))} Rust targets are outside the preserved Stage3 matrix. "
             f"{outside_review.get('target_level_reviews', 0)} have a target-level review and "
             f"{len(outside_review.get('unreviewed_targets', []))} still need one. The original law, prim and concurrency source checks remain required.",
             'Review all remaining targets and assertions, retain direct Rust API roles explicitly, and add every missing portable behavior case to final Stage3 execution.',link('bootstrap/current-outside-frontend-target-review.json')),
        item('gap:structural-projection','Replay anonymous refinement contracts','running' if projection.get('status') == 'running' else 'recheck',
             'The corrected target passed all 18 cases through audited Stage3. It checks explicit projection, refusal of implicit projection, and the original stronger-predicate refusal. Apply the fixture patch to the next integrated snapshot and rerun it.' if projection_proof else
             'The original sweep stops on an implicit projection from a structural refinement. Both compilers refuse that expression and accept explicit unrefine. The full replay includes both controls and retains the stronger-predicate refusal.',
             'The complete anonymous-contract target passes through matching Stage3, including explicit projection and the exact refusal reasons.',link('bootstrap/current-structural-projection-stage3-results.json' if projection_proof else 'compiler/tests/bootstrap_anonymous_contracts.rs')),
        item('gap:function-constructor','Replay higher-kinded function instances','recheck' if arrow_receipt else 'running',
             'The structural function constructor passes all three complete development groups. The unchanged Category family and matching final Stage3 still require their own results.' if arrow_receipt else
             'The original Category.Functor source needs the partially applied (->) type constructor.',
             'The original Category family and higher-kinded tests pass on matching Stage3.',link('bootstrap/current-structural-arrow-results.json' if arrow_receipt else 'compiler/prelude/Category/Functor.cinch')),
        item('gap:final-audit','Complete the requirement-by-requirement final audit','pending','A green subset, source check or earlier fixed point cannot close the full goal.',
             'Every acceptance item, named artifact, invariant and complete matching suite has direct current evidence; all failures and unclassified coverage are closed.',link('design/self-hosting.md#acceptance'))]

    active=[]
    for key, title in [
        ('partialBooleanBootstrapChain', 'Fresh Stage0 through Stage3 rebuild'),
        ('partialBooleanBootstrapPrior', 'Earlier Stage2 attempt before the library specialization'),
        ('partialBooleanBootstrapHeapPrior', 'Earlier Stage3 attempt at 256 MiB'),
    ]:
        next_chain = read(cfg.get(key))
        if not next_chain:
            continue
        status = next_chain.get('status', '')
        proof = read(root/'bootstrap/current-partial-boolean-chain-results.json')
        audited = (key == 'partialBooleanBootstrapChain' and status == 'passed-chain-awaiting-independent-audit'
                   and proof.get('status') == 'passed-bootstrap-chain-only'
                   and proof.get('compiler_source_sha256') == next_chain.get('compiler_source_sha256')
                   and proof.get('source_sha256') == next_chain.get('source_sha256')
                   and [r.get('compiler_sha256') for r in proof.get('stages', [])] ==
                       [r.get('compiler_sha256') for r in next_chain.get('stages', [])])
        if audited:
            status = 'passed-bootstrap-chain-only'
        active.append(dict(
            title=title,
            status='running' if status.startswith(('building-', 'checking-')) else status,
            phase=('Waiting for the compiler/object tests, the own-source check, and the nine component targets'
                   if status == 'waiting-matching-development-gates'
                   else 'Stage2 requested the missing Data.List.list-length [R] library specialization'
                   if key == 'partialBooleanBootstrapPrior'
                   else 'Stage3 exhausted the 256 MiB heap after emitting 44 matching interfaces'
                   if key == 'partialBooleanBootstrapHeapPrior'
                   else 'Building Stage3 with a 512 MiB heap after the verified 256 MiB exhaustion'
                   if status == 'building-stage3' and next_chain.get('heap_limit') == '512M'
                   else 'Independent audit passed, including changed compiler source'
                   if audited else status.replace('-', ' ')),
            passed=sum(stage['status'] == 'passed' for stage in next_chain.get('stages', [])),
            failed=int(status.startswith('failed')),
            total=4,
            unit='stages',
            source=next_chain.get('compiler_source_sha256') or read(cfg.get('partialBooleanValidation')).get('compiler_source_sha256'),
            auditPending=status == 'passed-chain-awaiting-independent-audit'))
    for key,title,total,skip in [
        ('nativeDefaultAnnotationValidation','Rust annotation repair: primary groups',14,'build-'),
        ('nativeDefaultAnnotationFollowup','Rust annotation repair: proof/syntax groups',7,'compile-'),
        ('sourceClassDefaultValidation','Cinch default implementation: complete component groups',4,'compile-'),
        ('sourceClassDefaultScopeValidation','Cinch annotation namespace repair: complete groups',7,'compile-'),
        ('sourceClassDefaultSelfValuesValidation','Cinch strict-default repair: complete groups',7,'compile-'),
        ('inferredCaseCoverageValidation','Cinch constructor coverage: complete groups',8,'compile-'),
        ('refinementContainerValidation','Cinch refined containers: complete groups',8,'compile-'),
        ('sourceDefaultDiagnosticValidation','Cinch class/instance diagnostics: complete groups',2,'compile-'),
        ('sourceDefaultOriginValidation','Cinch source-file diagnostics: complete groups',3,'compile-'),
        ('refinementExpectedResultValidation','Cinch structural result inference: complete groups',8,'compile-'),
        ('currentAuxiliaryValidation','Current Rust tooling and editor suites',2,'compile-'),
        ('sourceDefaultOriginInput','Diagnostic source: Native and earlier self-built checks',2,'build-'),
        ('refinementExpectedResultInput','Result-inference source: Native and earlier self-built checks',2,'build-'),
        ('partialBooleanValidation','Cinch partial Boolean implications: complete groups',9,'compile-'),
        ('partialBooleanInput','Core branch repair: complete Native44 source check',1,'build-'),
        ('sourceOriginOwnCompilerCheck','Diagnostic compiler: its own44 source modules',1,'build-'),
        ('partialBooleanOwnCompilerCheck','Core branch compiler: its own44 source modules',1,'build-'),
        ('sourceOriginByteScan','Diagnostic byte scan: large-bundle control and full pipeline',2,'compile-'),
        ('sourceOriginByteScanPrior','Diagnostic byte scan: earlier run with stale FFI expectation',2,'compile-'),
        ('constructorPublicDisplay','Constructor display: interface and nominal tests',2,'compile-'),
        ('constructorBaseDisplay','Base constructor spellings: complete exporter and nominal tests',2,'compile-'),
        ('originalConcurrencyFrontend','Original concurrency suite through the Cinch compiler',1,'compile-'),
        ('nativeTaskLocalCapture','Rust callback capture: focused, concurrency and unit tests',3,'compile-'),
        ('taskEffectDiagnostic','Cinch source effects: retain the Blocking label',1,'compile-'),
        ('taskTraceDiagnostics','Cinch task refusals: runtime-IR suite',1,'compile-'),
        ('taskTraceRuntimeReplay','Cinch task refusals: retained emitter, runtime setup and whole suite',3,'compile-'),
        ('taskDropDiagnostics','Cinch dropped-task diagnostic: complete runtime-IR suite',1,'compile-'),
        ('taskForwarderValidation','Cinch task forwarding: complete pipeline tests',1,'compile-'),
        ('stage3PromotionPipeline512','Current Stage3 original pipeline at 512 MiB',1,'compile-'),
        ('integratedTaskLawAdmission','Integrated compiler: Stage3 source admission',1,'compile-'),
        ('arrowFormerValidation','Structural function instances: pipeline, syntax and instance checks',3,'compile-'),
        ('structuralProjectionReplay','Stage3 anonymous refinements: explicit structural projection',1,'compile-'),
        ('nativeTaskForwarderSweep','Rust task-forwarding candidate: complete Rust sweep',235,'compile-'),
        ('sourceInterfaceOverlay','Source-library replacement: complete pipeline and dependency checks',1,'compile-'),
        ('originalSourceOverlay','Original four-family source-library replacement test',1,'compile-'),
        ('functionDefaultProvider','Function defaults: fresh pinned library object',1,'compile-'),
        ('additionalFixityValidation','Additional original fixity tests: actual Cinch frontend',1,'compile-')]:
        state=read(cfg.get(key))
        if state:
            wanted={'proof_guarded_lets','evidence_invalidation','index_local_carriers','syntax_index_evidence','syntax_index_lambdas','syntax_signature_scope','staging'} if 'Followup'in key else None
            rows=[r for r in state.get('rows',[])if not r['name'].startswith(skip)and(wanted is None or r['name']in wanted)]
            display_status=state.get('status')
            display_phase=state.get('phase')
            follow_key={'sourceClassDefaultScopeValidation':'sourceClassDefaultScopeFollowup','sourceClassDefaultSelfValuesValidation':'sourceClassDefaultSelfValuesFollowup'}.get(key)
            if follow_key:
                follow_state=read(cfg.get(follow_key))
                if follow_state:
                    assert follow_state.get('compiler_source_sha256')==state.get('compiler_source_sha256'), 'Followup compiler inputs differ'
                    real_follow=[r for r in follow_state.get('rows',[]) if not r['name'].startswith('compile-')]
                    rows=rows+real_follow
                    runner_error=any(r['name']in ('compile-bootstrap_source_value_aliases','compile-bootstrap_precondition_schemes') and r['status']=='failed' for r in state.get('rows',[]))
                    semantic_failure=any(r['status']=='failed' for r in rows)
                    if runner_error and not semantic_failure:
                        proof=read(cfg.get({'sourceClassDefaultScopeValidation':'sourceClassDefaultScopeComponentReceipt','sourceClassDefaultSelfValuesValidation':'sourceClassDefaultSelfValuesComponentReceipt'}[key]))
                        proof_matches=proof.get('compiler_source_sha256')==state.get('compiler_source_sha256') and proof.get('whole_groups')==total and proof.get('rust_tests')==9
                        display_status=('passed-audited' if proof_matches else 'passed-execution-awaiting-audit') if len(rows)==total and all(r['status']=='passed' for r in rows) else 'running'
                        display_phase='Runner target-name error preserved; remaining real groups: '+follow_state.get('phase',follow_state.get('status','queued'))
            receipt_key={'sourceDefaultDiagnosticValidation':'sourceDefaultDiagnosticComponentReceipt','inferredCaseCoverageValidation':'inferredCaseCoverageComponentReceipt','refinementContainerValidation':'refinementContainerComponentReceipt','refinementExpectedResultValidation':'refinementExpectedResultComponentReceipt','currentAuxiliaryValidation':'currentAuxiliaryReceipt','partialBooleanValidation':'partialBooleanComponentReceipt','partialBooleanInput':'partialBooleanInputReceipt','sourceDefaultOriginValidation':'sourceDefaultOriginComponentReceipt','sourceOriginOwnCompilerCheck':'sourceOriginOwnCompilerReceipt','additionalFixityValidation':'additionalFixityReceipt','partialBooleanOwnCompilerCheck':'partialBooleanOwnCompilerReceipt','functionDefaultProvider':'functionDefaultProviderReceipt'}.get(key)
            completed_receipt_keys = {
                'sourceOriginByteScan': 'sourceOriginByteScanReceipt',
                'constructorPublicDisplay': 'constructorPublicDisplayReceipt',
                'nativeTaskLocalCapture': 'nativeTaskLocalCaptureReceipt',
                'taskEffectDiagnostic': 'taskEffectDiagnosticReceipt',
                'taskTraceRuntimeReplay': 'taskTraceRuntimeReplayReceipt',
                'taskDropDiagnostics': 'taskDropDiagnosticsReceipt',
                'stage3PromotionPipeline512': 'stage3PromotionPipeline512Receipt',
                'integratedTaskLawAdmission': 'integratedTaskLawAdmissionReceipt',
                'constructorBaseDisplay': 'constructorBaseDisplayReceipt',
                'sourceInterfaceOverlay': 'sourceInterfaceOverlayReceipt',
            }
            receipt_key = completed_receipt_keys.get(key, receipt_key)
            receipt=read(cfg.get(receipt_key)) if receipt_key else annotation_receipt if key.startswith('nativeDefaultAnnotation') else default_receipt
            audited=bool(receipt) and receipt.get('compiler_source_sha256')==state.get('compiler_source_sha256') and (receipt.get('source')==state.get('source') or receipt.get('whole_groups')==total)
            if key == 'nativeTaskLocalCapture':
                audited = bool(receipt) and receipt.get('native_source_sha256') == state.get('native_source_sha256')
            active.append(dict(title=title,status=display_status,phase=display_phase,passed=sum(r['status']=='passed'for r in rows),failed=sum(r['status']=='failed'for r in rows),total=total,source=state.get('compiler_source_sha256'),nativeSource=state.get('native_source_sha256'), auditPending=state.get('status','').startswith('passed-') and not audited))

    for key,title,total in [('sourceClassDefaultFullCompiler','Cinch defaults: full compiler/object gate',2), ('nativeDefaultAnnotationApi','Published Cinch: Rust reader/API replay',9), ('pipelineInterfaceDiagnosticValidation','Pipeline: complete FFI and mode checks',1), ('sourceClassDefaultApi','Cinch defaults: original API and default groups',10), ('inferredCaseCoverageFullCompiler','Cinch constructor coverage: full compiler/object gate',2), ('refinementStructuralFullCompiler','Cinch structural refinements: full compiler/object gate',2), ('refinementStructuralApi','Cinch structural refinements: original API/default groups',10), ('sourceDefaultOriginFullCompiler','Cinch diagnostic origins: full compiler/object gate',2), ('sourceDefaultOriginApi','Cinch diagnostic origins: original API/default groups',10), ('refinementExpectedResultFullCompiler','Cinch result inference: full compiler/object gate',2), ('partialBooleanFullCompiler','Cinch Core branch implications: full compiler/object gate',2), ('refinementExpectedResultApi','Cinch result inference: original API/default groups',10), ('functionDefaultApi','Cinch defaults: full replay with function factories',10)]:
        state=read(cfg.get(key))
        if state:
            rows=[r for r in state.get('rows',[]) if not r['name'].startswith(('compile-','build-'))]
            proof_key={'sourceClassDefaultFullCompiler':'sourceClassDefaultCompilerReceipt','nativeDefaultAnnotationApi':'interfaceApiReceipt','pipelineInterfaceDiagnosticValidation':'nativePipelineReceipt','sourceClassDefaultApi':'sourceClassDefaultApiReceipt','inferredCaseCoverageFullCompiler':'inferredCaseCoverageCompilerReceipt','refinementStructuralFullCompiler':'refinementStructuralCompilerReceipt','refinementStructuralApi':'refinementStructuralApiReceipt','sourceDefaultOriginFullCompiler':'sourceDefaultOriginCompilerReceipt','sourceDefaultOriginApi':'sourceDefaultOriginApiReceipt','refinementExpectedResultFullCompiler':'refinementExpectedResultCompilerReceipt','partialBooleanFullCompiler':'partialBooleanCompilerReceipt','refinementExpectedResultApi':'refinementExpectedResultApiReceipt','functionDefaultApi':'functionDefaultApiReceipt'}[key]
            proof=read(cfg.get(proof_key))
            audited=bool(proof) and proof.get('source_sha256')==state.get('source_sha256')
            active.append(dict(title=title,status=state.get('status'),phase=state.get('phase'),passed=sum(r['status']=='passed'for r in rows),failed=sum(r['status']=='failed'for r in rows),total=total,source=state.get('compiler_source_sha256'),auditPending=state.get('status')=='passed' and not audited))

    for title, state, total in [('Current compiler: full Rust sweep' if full.get('compiler_source_sha256')==compiler and full.get('native_source_sha256')==native else 'Earlier compiler: full Rust sweep', full, len(native_names)),
                                ('Earlier Stage3: complete frontend sweep', front, len(front_names))]:
        rows = state.get('rows', [])
        if state is full and read(cfg.get('nativeArrayFixtureReceipt')).get('source_sha256')==full.get('source_sha256'):
            rows=[dict(r,status='passed') if r['name']=='arrays' and r['status']=='failed' else r for r in rows]
        if state.get('status') == 'running':
            active.append(dict(title=title, status=state['status'], phase=state.get('phase'),
                passed=sum(r.get('status') == 'passed' for r in rows),
                failed=sum(r.get('status') == 'failed' for r in rows), total=total,
                source=state.get('compiler_source_sha256'), earlier=title.startswith('Earlier')))

    integrated_chain=read(cfg.get('integratedTaskLawChain'))
    if integrated_chain:
        state=integrated_chain.get('status','')
        completed=sum(row['status']=='passed' for row in integrated_chain.get('stages',[]))
        finished=state.startswith('passed')
        audited=(integrated_proof.get('status')=='passed-bootstrap-chain-only'
                 and integrated_proof.get('compiler_source_sha256')==integrated_chain.get('compiler_source_sha256')
                 and integrated_proof.get('source_sha256')==integrated_chain.get('source_sha256'))
        active.insert(0, dict(title='Integrated compiler: fresh bootstrap', status='running' if state.startswith(('building-', 'checking-')) else state,
            phase='Independent audit passed, including matching Stage2 and Stage3 and the changed-source control' if audited else state.replace('-', ' '),
            passed=completed, failed=int(state.startswith('failed')), total=4,
            source=integrated_chain.get('compiler_source_sha256'), auditPending=finished and not audited))

    if cinch_forwarder.get('status') == 'verified-task-forwarder-six-case-development-replay':
        active.append(dict(title='Cinch task forwarding: six-case source replay', status='passed',
            phase='Unchanged Tasks program and refusal, unsafe capture, and interface boundary',
            passed=1, failed=0, total=1, source=cinch_forwarder['compiler_source_sha256']))
    if native_forwarder.get('status') == 'verified-native-task-forwarder-development':
        active.append(dict(title='Rust task forwarding: complete focused tests', status='passed',
            phase='19 concurrency tests and 186 compiler unit tests',
            passed=2, failed=0, total=2, source=None))

    candidate_external = read(root/'bootstrap/current-partial-boolean-external-acceptance-results.json')
    if candidate_external.get('status') == 'passed-independent-stage3-external-acceptance':
        count = candidate_external['original_acceptance_checks']
        active.append(dict(title='Current candidate: external Stage3 acceptance', status='passed',
            phase='Independently verified original runtime, fixity and class identity checks',
            passed=count, failed=0, total=count, source=candidate_external['compiler_source_sha256']))

    if language.get('status', '').startswith('audited-matching-stage3-external-behavior'):
        passed_cases = language['positive_passed'] + language['negative_intended_reason_passed']
        active.append(dict(title='Current candidate: Stage3 language examples', status='completed-with-failures' if passed_cases != 34 else 'passed',
            phase=f"{language['positive_passed']}/13 valid programs and {language['negative_intended_reason_passed']}/21 exact refusal checks",
            passed=passed_cases, failed=34-passed_cases, total=34, source=language['compiler_source_sha256']))

    for key, title in (
        ('currentStage3FrontendValidation', 'Earlier self-built Stage3: complete frontend sweep'),
        ('integratedStage3FrontendValidation', 'Integrated self-built Stage3: complete frontend sweep'),
        ('schema9Stage3FrontendValidation', 'Earlier schema 9 Stage3: all 105 frontend targets'),
    ):
        candidate_frontend = read(cfg.get(key))
        if not candidate_frontend:
            continue
        rows = candidate_frontend.get('rows', [])
        status = candidate_frontend.get('status')
        active.insert(1, dict(title=title, status=status,
            phase=candidate_frontend.get('phase') or ('Waiting for independent Stage3 audit' if status == 'waiting-matching-stage3-audit' else None),
            passed=sum(r.get('status') == 'passed' for r in rows),
            failed=sum(r.get('status') == 'failed' for r in rows),
            total=candidate_frontend['total_groups'], source=candidate_frontend.get('compiler_source_sha256')))

    for title, run, total in (
        ('Cinch copyOut: pipeline and runtime IR', copyout, 2),
        ('Cinch class-law samples and counterexamples', laws, 2),
        ('Cinch imported defaults: source admission and two complete groups', default_transport, 3),
        ('Schema 9: source admission, imported defaults, runtime and pipeline', schema9_validation, 5),
        ('Category: class type and dictionary export parity', class_interface, 1),
        ('Rust: preserve the structural arrow in instance contracts', class_arrow_native, 2),
        ('Schema 9 Stage1: original four-family replacement test', schema9_original_overlays, 1),
        ('Cinch: checked compile-time evaluation', stage_evaluation, 1),
        ('Cinch: staged generation through the full pipeline', stage_expansion, 1),
        ('Cinch: retain defining nominal dependencies in inherited defaults and interface exports', nominal_dependency, 2),
        ('Rust: emit definition-owned instance headers', instance_header, 2),
        ('Cinch: finite class laws and inherited default closure', finite_law, 2),
        ('Schema 10: original separate class-law replay', portable_law, 1),
        ('Schema 10: corrected law decoder and typed constructor samples', typed_law, 1),
        ('Rust: Nat-backed law counterexample and interface tests', native_typed_law if not native_typed_law_proof else {}, 1),
        ('Schema 9 Stage1: initial arrays run before the runtime fixture retry', read(cfg.get('schema9StagingCopyOutArrays')), 2),
        ('Schema 10: pinned Stage1 and original library replacement tests', read(cfg.get('schema10OrderingStage1')), 8),
        ('Schema 9: rebuild and check the Git-pinned provider', schema9_pinned_provider, 1),
    ):
        if run:
            rows = [r for r in run.get('rows', []) if not r['name'].startswith(('compile-', 'build-'))]
            active.append(dict(title=title, status=run['status'], phase=run.get('phase') or run['status'].replace('-', ' '),
                passed=sum(r['status'] == 'passed' for r in rows), failed=sum(r['status'] == 'failed' for r in rows),
                total=total, source=run.get('native_source_sha256' if run is class_arrow_native or run is instance_header or run is native_typed_law else 'compiler_source_sha256')))
    schema10_provider = read(cfg.get('schema10LibraryProvider'))
    if schema10_provider:
        status = schema10_provider['status']
        active.append(dict(title='Schema 10: rebuild the library with checked class laws', status=status,
            phase=status.replace('-', ' '), passed=int(status.startswith('passed')), failed=int(status.startswith('failed')),
            total=1, source=schema10_provider.get('producer_source_sha256')))
    corrected_provider = read(cfg.get('schema10CorrectedLibraryProvider'))
    if corrected_provider:
        status = corrected_provider['status']
        active.append(dict(title='Schema 10: rebuild the library after the Rust numeric-law fix', status=status,
            phase=status.replace('-', ' '), passed=int(status.startswith('passed')), failed=int(status.startswith('failed')),
            total=1, source=corrected_provider.get('producer_source_sha256')))
    if native_typed_law_proof.get('status') == 'passed-native-mathematical-law-sampling':
        active.append(dict(title='Rust: Nat-backed law counterexample and interface tests', status='passed',
            phase='Reject the wrong identity and evaluate the lawful Nat-backed samples',
            passed=native_typed_law_proof['passed'], failed=0, total=native_typed_law_proof['total'],
            source=native_typed_law_proof['native_source_sha256'], evidence=link('bootstrap/current-native-mathematical-law-sampling-results.json')))
    if native_numeric_method_proof.get('status') == 'passed-native-numeric-and-class-method-contracts':
        active.append(dict(title='Rust: mathematical law equations and canonical class methods', status='passed',
            phase='Four complete targets across the original run and the corrected law assertion replay',
            passed=native_numeric_method_proof['passed'], failed=0, total=native_numeric_method_proof['total'],
            source=native_numeric_method_proof['native_source_sha256'], evidence=link('bootstrap/current-native-numeric-and-class-method-contract-results.json')))
    typed_admission = read(cfg.get('schema10TypedLawAdmission'))
    if typed_admission:
        checked = next((row for row in typed_admission.get('rows', []) if row['name'] == 'check-44-compiler-modules'), {})
        status = checked.get('status', typed_admission['status'])
        active.append(dict(title='Schema 10: check all 44 compiler modules with the pinned Rust seed', status=status,
            phase=status.replace('-', ' '), passed=int(status == 'passed'), failed=int(status == 'failed'), total=1,
            source=typed_admission['compiler_source_sha256']))
    if schema10_integrated_proof.get('status') == 'audited-schema10-stage1-parity-and-integrated-inputs':
        proven = schema10_integrated_proof['combined_candidate']
        active.append(dict(title='Schema 10: combined source check and Git-pinned library', status='passed',
            phase=f"The pinned seed accepted all {proven['compiler_modules']} modules and the normal library build matched all {proven['library_interfaces']} interfaces",
            passed=2, failed=0, total=2, source=proven['compiler_source_sha256'],
            evidence=link('bootstrap/current-schema10-stage1-and-integrated-input-results.json')))
    schema10_production = read(cfg.get('schema10ProductionChain'))
    stage3_audit_path = 'bootstrap/current-schema10-stage3-and-library-replays-results.json'
    stage3_audit = read(root/stage3_audit_path)
    production_audit_path = 'bootstrap/current-schema10-bootstrap-results.json'
    production_audit = read(root/production_audit_path)
    if schema10_production:
        status = schema10_production['status']
        phase = status.replace('-', ' ')
        stage2_audit_path = 'bootstrap/current-schema10-stage2-and-source-laws-results.json'
        stage2_audit = read(root/stage2_audit_path)
        audited_source = stage2_audit.get('schema10_stage2', {}).get('compiler_source_sha256')
        if stage3_audit.get('schema10_stage_agreement', {}).get('compiler_source_sha256') == schema10_production['compiler_source_sha256']:
            stage2_audit_path = stage3_audit_path
            audited_source = schema10_production['compiler_source_sha256']
        if (production_audit.get('status') == 'passed-production-schema10-bootstrap-chain' and
                production_audit.get('compiler_source_sha256') == schema10_production['compiler_source_sha256']):
            stage2_audit_path = production_audit_path
            audited_source = schema10_production['compiler_source_sha256']
            status = 'passed'
            phase = 'The fixed point, separate interface consumer and changed-source rebuild passed independent audit.'
        active.append(dict(title='Schema 10: combined self-host chain', status=status, phase=phase,
            passed=sum(row.get('status') == 'passed' for row in schema10_production.get('stages', [])),
            failed=int(status.startswith('failed')), total=4, source=schema10_production['compiler_source_sha256'],
            evidence=link(stage2_audit_path) if audited_source == schema10_production['compiler_source_sha256'] else None))
        source_laws = stage2_audit.get('schema9_source_laws', {})
        if source_laws:
            passed_law_tests = sum(row['tests'] for row in source_laws['targets'].values())
            active.append(dict(title='Finite laws and source-instance regression tests', status='passed',
                phase='Both pipeline tests and all three source-instance tests passed on the audited schema 9 candidate.',
                passed=passed_law_tests, failed=0, total=passed_law_tests, source=source_laws['compiler_source_sha256'],
                evidence=link(stage2_audit_path)))
    remaining_family = read(cfg.get('schema10RemainingFamilyValidation'))
    if remaining_family:
        rows = remaining_family.get('rows', [])
        active.append(dict(title='Schema 10: division and canonical value schemes', status=remaining_family['status'],
            phase=remaining_family.get('phase') or remaining_family['status'].replace('-', ' '),
            passed=sum(row.get('status') == 'passed' for row in rows),
            failed=sum(row.get('status') == 'failed' for row in rows), total=2, source=remaining_family['compiler_source_sha256']))
    canonical_laws = read(cfg.get('schema10LawIdentitiesValidation'))
    law_identity_gap = read(root/'bootstrap/current-schema10-law-identity-gap.json')
    if law_identity_gap:
        rows = canonical_laws.get('rows', [])
        status = canonical_laws.get('status', 'pending')
        active.append(dict(title='Match defining names in schema 10 law observations', status=status,
            phase='Rust and Cinch emitted different law names in the same schema. ' +
                (canonical_laws.get('phase') or status).replace('-', ' '),
            passed=sum(row.get('status') == 'passed' for row in rows),
            failed=sum(row.get('status') == 'failed' for row in rows), total=5,
            source=canonical_laws.get('compiler_source_sha256'),
            evidence=link('bootstrap/current-schema10-law-identity-gap.json')))
    envelope = read(cfg.get('schema10InterfaceEnvelopeValidation'))
    interface_audit_path = 'bootstrap/current-schema10-interface-controls-and-schema9-nonzero-results.json'
    interface_audit = read(root/interface_audit_path)
    if envelope:
        rows = envelope.get('rows', [])
        active.append(dict(title='Require complete schema 10 interfaces and preserve public type spelling',
            status=envelope['status'], phase=(envelope.get('phase') or envelope['status']).replace('-', ' '),
            passed=sum(row.get('status') == 'passed' for row in rows),
            failed=sum(row.get('status') == 'failed' for row in rows), total=9,
            source=envelope['compiler_source_sha256'],
            evidence=link(interface_audit_path) if interface_audit.get('schema10', {}).get('compiler_source_sha256') == envelope['compiler_source_sha256'] else None))
    nonzero_replay = interface_audit.get('schema9_stage3_nonzero', {})
    if nonzero_replay.get('status') == 'passed-original-stage3-structural-nonzero':
        active.append(dict(title='Stage3 structural NonZero with the missing library factory', status='passed',
            phase='The unchanged test passed using the same compiler, runtime and 55 interface identities.',
            passed=nonzero_replay['passed'], failed=0, total=nonzero_replay['total'],
            source=nonzero_replay['compiler_source_sha256'], evidence=link(interface_audit_path)))
    successor = read(cfg.get('schema10SuccessorChain'))
    if successor:
        status = successor['status']
        published = successor.get('compiler_source_sha256') == compiler
        audited = chain is strict_audit and strict_audit.get('native_source_sha256') == successor.get('native_source_sha256')
        active.insert(0, dict(title='Bootstrap the schema 10 compiler on main' if published else 'Bootstrap the stricter schema 10 candidate',
            status='passed' if audited else 'running' if status.startswith(('building-', 'checking-')) else status,
            phase='The independent fixed-point and changed-source audit passed.' if audited else status.replace('-', ' '),
            passed=sum(row.get('status') == 'passed' for row in successor.get('stages', [])),
            failed=int(status.startswith('failed')), total=4, unit='stages', source=successor['compiler_source_sha256'],
            auditPending=status == 'passed-production-schema10-chain-awaiting-independent-audit' and not audited,
            evidence=link(strict_audit_path) if audited else None))
    families = read(cfg.get('schema10SuccessorStage1Families'))
    if families:
        status = families['status']
        active.append(dict(title='Replay the original library families with the rebuilt provider', status=status,
            phase=(families.get('phase') or status).replace('-', ' '),
            passed=int(status == 'passed-original-schema10-family-overlays'), failed=int(status.startswith('failed')),
            total=1, source=families['compiler_source_sha256'],
            evidence=link(family_audit_path) if family_audit.get('compiler_source_sha256') == families['compiler_source_sha256'] else None))
    if cutover.get('status') == 'passed':
        active.append(dict(title='Check the schema 10 cutover in Rust, Cinch and Python', status='passed',
            phase='The real schema 9 file, the old default encoding and both malformed schema 10 files refused.',
            passed=len(cutover['cases']), failed=0, total=len(cutover['cases']),
            source=family_audit['compiler_source_sha256'], evidence=link(family_audit_path)))
    provider_replay = stage3_audit.get('schema10_successor_provider', {})
    if provider_replay.get('status') == 'passed-production-schema10-provider':
        active.append(dict(title='Rebuild the stricter schema 10 library', status='passed',
            phase='Verified 55 library interfaces and matched all five Applicative display spellings.',
            passed=55, failed=0, total=55, source=provider_replay['compiler_source_sha256'], evidence=link(stage3_audit_path)))
    body_replay = stage3_audit.get('schema9_stage3_body_bindings', {})
    if body_replay.get('status') == 'passed-original-stage3-body-bindings':
        active.append(dict(title='Stage3 body binding with the missing library factory', status='passed',
            phase='Both original assertions passed with the same compiler, runtime and 55 interface identities.',
            passed=body_replay['passed'], failed=0, total=body_replay['total'],
            source=body_replay['compiler_source_sha256'], evidence=link(stage3_audit_path)))
    language = read(cfg.get('schema10Stage3Language'))
    language_audit_path = 'bootstrap/current-schema10-external-language-results.json'
    language_audit = read(root/language_audit_path)
    if language:
        status = language['status']
        audited = (language_audit.get('compiler_source_sha256') == language['compiler_source_sha256']
                   and language_audit.get('compiler_sha256') == language.get('compiler_sha256')
                   and language_audit.get('original_cases') == 34)
        passed_cases = 34 if status == 'passed' else None
        phase = status.replace('-', ' ')
        if audited:
            passed_cases = language_audit['positive_passed'] + language_audit['negative_intended_reason_passed']
            phase = (f"{language_audit['positive_passed']}/13 valid programs passed. "
                     f"All {language_audit['negative_refused']} invalid programs refused. "
                     f"{language_audit['negative_intended_reason_passed']}/21 met the original diagnostic assertions.")
        active.append(dict(title='First schema 10 Stage3: all 34 original language examples', status=status,
            phase=phase, passed=passed_cases, failed=34-passed_cases if passed_cases is not None else None,
            total=34, source=language['compiler_source_sha256'], evidence=link(language_audit_path) if audited else None))
    for key, title, total in (
        ('schema10ExternalLibraryVariants', 'Rebuild the libraries with five requested runtime variants', 2),
        ('schema10ExternalVariantReplay', 'First library replay of Html, Reactive and TensorAD', 3),
        ('schema10ExternalVariantReplay2', 'Replay the same three programs with the additional method and zip variants', 3),
        ('schema10ExternalVariantReplay3', 'Replay the three programs with both required Functor variants', 3),
        ('schema10MainTaskValidation', 'Rust task capture, units and arrays on schema 10', 3),
        ('schema10DiagnosticScanValidation', 'Replay the full pipeline with the diagnostic byte scan', 2),
        ('schema10DiagnosticScanPipelineReplay', 'Retry the byte-scan pipeline with both runtime archives', 1),
        ('schema10ProofReflexivityValidation', 'Check erased Eq witnesses in Core and the source pipeline', 2),
        ('schema10NamedProofValidation', 'Check closed named proofs and their original runtime example', 2),
    ):
        run = read(cfg.get(key))
        if run:
            rows = [row for row in run.get('rows', []) if not row['name'].startswith(('build-', 'compile-'))]
            active.append(dict(title=title, status=run['status'],
                phase=(run.get('phase') or run['status']).replace('-', ' '),
                passed=sum(row.get('status') == 'passed' for row in rows),
                failed=sum(row.get('status') == 'failed' for row in rows), total=total,
                source=run.get('compiler_source_sha256')))
    diagnostic_probe = read(cfg.get('schema10Stage2DiagnosticProbe'))
    byte_observation_path = 'bootstrap/current-schema10-stage2-byte-observation-results.json'
    byte_observation = read(root / byte_observation_path)
    if diagnostic_probe:
        status = diagnostic_probe['status']
        exit_code = diagnostic_probe.get('exit_code')
        if exit_code is not None:
            status = 'passed' if exit_code == 0 else 'failed'
        diagnosed = byte_observation.get('diagnostic_sha256') == diagnostic_probe.get('diagnostic_sha256') and bool(byte_observation)
        active.append(dict(title='Probe the original Stage2 source with faster diagnostics', status=status,
            phase='Stage1 refused conflicting type spellings for the imported bytes-len intrinsic.' if diagnosed else
                  (diagnostic_probe.get('phase') or diagnostic_probe['status']).replace('-', ' '),
            passed=int(bool(diagnostic_probe.get('stage1_sha256'))) + int(exit_code == 0),
            failed=int(exit_code not in (None, 0)), total=2,
            source=diagnostic_probe['compiler_source_sha256'], evidence=link(byte_observation_path) if diagnosed else None))
    byte_repair = read(cfg.get('schema10ByteObservationRepair'))
    if byte_repair:
        status = byte_repair['status']
        byte_repair_path = 'bootstrap/current-schema10-byte-observation-repair-results.json'
        repair_audit = read(root / byte_repair_path)
        verified = (repair_audit.get('status') == 'audited-repaired-schema10-byte-observation-provider-and-client'
                    and repair_audit.get('producer') == byte_repair.get('producer')
                    and repair_audit.get('compiler_source_sha256') == byte_repair['compiler_source_sha256'])
        active.append(dict(title='Rebuild the pinned library and rerun the ladder with one bytes-len type spelling',
            status=status, phase=byte_repair['phase'], passed=len(byte_repair['rows']),
            failed=int(status == 'failed'), total=3, source=byte_repair['compiler_source_sha256'],
            auditPending=status == 'passed-ladder-awaiting-independent-audit',
            evidence=link(byte_repair_path) if verified else None))
    byte_tests = read(cfg.get('schema10ByteObservationTests'))
    if byte_tests:
        status = byte_tests['status']
        active.append(dict(title='Run all original byte-observation tests with the repaired producer',
            status='passed' if status == 'passed-complete-byte-observation-target' else status,
            phase=byte_tests['phase'], passed=int(status == 'passed-complete-byte-observation-target'),
            failed=int(status.startswith('failed')), total=1, source=byte_tests.get('compiler_source_sha256')))
    repaired_stage2_path = 'bootstrap/current-schema10-repaired-stage2-results.json'
    repaired_stage2 = read(root / repaired_stage2_path)
    if repaired_stage2.get('status') == 'audited-repaired-schema10-stage2':
        active.append(dict(title='Build and smoke-test Stage2 with the repaired library', status='passed',
            phase='Stage1 rebuilt all 46 compiler modules and Stage2 passed all three smoke cases.',
            passed=1, failed=0, total=1, source=repaired_stage2['compiler_source_sha256'],
            evidence=link(repaired_stage2_path)))
    repaired_agreement_path = 'bootstrap/current-schema10-repaired-stage3-agreement-results.json'
    repaired_agreement = read(root / repaired_agreement_path)
    if repaired_agreement.get('status') == 'audited-repaired-schema10-stage3-agreement':
        active.append(dict(title='Verify Stage2 and Stage3 agree on the repaired compiler', status='passed',
            phase='Executables, LLVM IR and 46 interfaces agree. Stage3 passed the smoke tests and a separate client.',
            passed=1, failed=0, total=1, source=repaired_agreement['compiler_source_sha256'],
            evidence=link(repaired_agreement_path)))
    consolidated_path = 'bootstrap/current-schema10-consolidated-provider-results.json'
    consolidated = read(root / consolidated_path)
    if consolidated.get('status') == 'audited-consolidated-provider-and-complete-runtime-tooling':
        active.append(dict(title='Verify the consolidated library, runtime and tooling', status='passed',
            phase='All 55 interfaces, the Stage1 client, 171 runtime tests and 42 tooling tests passed.',
            passed=4, failed=0, total=4, source=consolidated['compiler_source_sha256'],
            evidence=link(consolidated_path)))
    native_sweep = read(cfg.get('schema10ConsolidatedNativeSweep'))
    if native_sweep:
        status = native_sweep['status']
        rows = native_sweep['rows']
        active.append(dict(title='Run every Rust compiler target on the combined fixes',
            status='passed' if status == 'passed-complete-native-runtime-and-tooling' else status,
            phase=native_sweep.get('phase', 'Waiting for a worker'),
            passed=sum(row['status'] == 'passed' for row in rows),
            failed=sum(row['status'] == 'failed' for row in rows), total=native_sweep['total_groups'],
            source=native_sweep['native_source_sha256']))
    proof_chain = read(cfg.get('schema10ConsolidatedProofChain'))
    if proof_chain:
        status = proof_chain['status']
        active.append(dict(title='Bootstrap the consolidated compiler with named proofs',
            status='running' if status.startswith(('building-', 'checking-')) else status,
            phase=status.replace('-', ' '), passed=len(proof_chain['stages']),
            failed=int(status == 'failed'), total=4, source=proof_chain['compiler_source_sha256'],
            auditPending=status == 'passed-consolidated-proof-ladder-awaiting-independent-audit'))
    reflexivity_path = 'bootstrap/current-schema10-reflexivity-core-results.json'
    reflexivity = read(root/reflexivity_path)
    if reflexivity.get('status') == 'passed-development-reflexivity-core':
        active.append(dict(title='Core checks erased Eq witnesses', status='passed',
            phase='The full original Core target passed with six additional assertions across all five runtime nonces.',
            passed=1, failed=0, total=1, source=reflexivity['compiler_source_sha256'],
            evidence=link(reflexivity_path)))
    proof_replay_path = 'bootstrap/current-schema10-original-proof-replay-results.json'
    proof_replay = read(root / proof_replay_path)
    if proof_replay.get('status') == 'audited-original-proof-examples-on-development-compiler':
        active.append(dict(title='Run both original Proof examples on the development compiler', status='passed',
            phase='The valid proof ran with zero leaks and the invalid proof reported the expected reason and source location.',
            passed=len(proof_replay['rows']), failed=0, total=2, source=proof_replay['compiler_source_sha256'],
            evidence=link(proof_replay_path)))
    replay_path = 'bootstrap/current-schema10-external-replay-results.json'
    replay = read(root/replay_path)
    if replay.get('status') == 'audited-external-language-replays':
        runtime = replay['stage3_runtime']
        active.append(dict(title='Stage3 TensorAD with the requested library variants', status='passed',
            phase='The unchanged program passed at 32 MiB with poisoning and zero leaks.',
            passed=sum(row['status'] == 'passed' for row in runtime['cases']), failed=0, total=1,
            source=runtime['compiler_source_sha256'], evidence=link(replay_path)))
        diagnostics = replay['development_stage1_diagnostics']
        active.append(dict(title='Stage1 Html and TensorPrivate refusals', status='passed',
            phase='Both original refusal reasons and original source locations matched.',
            passed=len(diagnostics['cases']), failed=0, total=2,
            source=diagnostics['compiler_source_sha256'], evidence=link(replay_path)))
    expanded_path = 'bootstrap/current-schema10-expanded-runtime-replays-results.json'
    expanded = read(root / expanded_path)
    if expanded.get('status') == 'audited-three-original-stage3-runtime-replays-and-stage1-diagnostics':
        runtime = expanded['stage3_runtime']
        active.append(dict(title='Stage3 runs Html, Reactive and TensorAD with the expanded library', status='passed',
            phase='All three original programs passed at 32 MiB with poisoning and zero leaks.',
            passed=len(runtime['cases']), failed=0, total=3,
            source=runtime['compiler_source_sha256'], evidence=link(expanded_path)))
    renderer_path = 'bootstrap/current-schema10-diagnostic-renderer-results.json'
    renderer = read(root / renderer_path)
    if renderer.get('status') == 'audited-complete-schema10-diagnostic-pipeline-target':
        active.append(dict(title='Verify the diagnostic renderer in the complete pipeline target', status='passed',
            phase='Both tests passed with the matching debug and release runtime archives.',
            passed=renderer['tests_passed'], failed=0, total=2,
            source=renderer['compiler_source_sha256'], evidence=link(renderer_path)))
    external_acceptance = read(cfg.get('schema10Stage3ExternalAcceptance'))
    external_audit_path = 'bootstrap/current-schema10-external-acceptance-results.json'
    external_audit = read(root/external_audit_path)
    if external_acceptance:
        status = external_acceptance['status']
        rows = external_acceptance.get('rows', [])
        active.append(dict(title='Run the original 14 external acceptance checks on schema 10 Stage3', status=status,
            phase=(external_acceptance.get('active') or status).replace('-', ' '),
            passed=sum(row.get('status') == 'passed' for row in rows),
            failed=sum(row.get('status') == 'failed' for row in rows), total=14,
            source=external_acceptance.get('compiler_source_sha256', production_audit.get('compiler_source_sha256')),
            evidence=link(external_audit_path) if external_audit.get('result_sha256') and status == 'passed' else None))
    extra_library = read(cfg.get('schema10AdditionalLibraryFrontend'))
    if extra_library:
        status = extra_library['status']
        rows = extra_library.get('rows', [])
        active.append(dict(title='Run four more original library and numeric targets with Stage3', status=status,
            phase='Forty original tests retain their fixtures and assertions and use the selected frontend.',
            passed=sum(row.get('status') == 'passed' for row in rows),
            failed=sum(row.get('status') == 'failed' for row in rows), total=4,
            source=extra_library['compiler_source_sha256']))
    arrays_audit_path = 'bootstrap/current-schema10-arrays-and-law-identity-results.json'
    arrays_audit = read(root/arrays_audit_path).get('original_arrays', {})
    if arrays_audit.get('passed') == 15:
        active.append(dict(title='Replay the original arrays with a custom Cargo output directory', status='passed',
            phase='All 15 original tests passed with the runtime archive placed where the linker reads it.',
            passed=15, failed=0, total=15, source=arrays_audit['compiler_source_sha256'], evidence=link(arrays_audit_path)))
    strict_shards = [read(path) for path in cfg.get('schema10StrictFrontendShards', [])]
    if strict_shards and all(shard.get('assigned_targets') for shard in strict_shards):
        assignments = [name for shard in strict_shards for name in shard['assigned_targets']]
        sources = {shard['compiler_source_sha256'] for shard in strict_shards}
        scope = read(root/'bootstrap/current-frontend-acceptance-scope.json')
        required = set(scope.get('original_targets', [])) | set(scope.get('new_targets_beyond_original', []))
        if len(sources) == 1 and len(assignments) == len(required) == 105 and set(assignments) == required:
            rows = [row for shard in strict_shards for row in shard.get('rows', [])]
            passed = sum(row.get('status') == 'passed' for row in rows)
            failed = sum(row.get('status') == 'failed' for row in rows)
            if passed == 105:
                status = 'passed'
            elif any(shard['status'].startswith('failed') for shard in strict_shards):
                status = 'failed-prerequisite'
            elif passed + failed == 105:
                status = 'completed-with-failures'
            else:
                status = 'running' if rows else 'waiting-stage3-audit'
            active.append(dict(title='Run the full original 105-target Stage3 frontend matrix', status=status,
                phase=f'{len(strict_shards)} disjoint runs share the same audited compiler and provider.',
                passed=passed, failed=failed, total=105, source=next(iter(sources))))
    if combined_arrays:
        active.append(dict(title='Schema 9 Stage1: original arrays with staging and CopyOut', status='passed',
            phase='Fourteen tests passed in the full run and the remaining test passed with the matching runtime archive',
            passed=combined_arrays['passed'], failed=0, total=combined_arrays['total'],
            source=combined_arrays['compiler_source_sha256'], evidence=link('bootstrap/current-stage1-combined-arrays-results.json')))
    if allocation_replay.get('status') == 'passed-original-stage3-allocation-accounting':
        active.append(dict(title='Schema 9 Stage3: allocation accounting with the missing library variant', status='passed',
            phase='The unchanged test binary passed with Data.List.length@T and all 55 original interface identities',
            passed=allocation_replay['passed'], failed=0, total=allocation_replay['total'],
            source=allocation_replay['compiler_source_sha256'], evidence=link('bootstrap/current-stage3-allocation-object-replay-results.json')))
    if finite_law_pipeline.get('status') == 'passed-complete-schema9-candidate-pipeline':
        active.append(dict(title='Schema 9: complete original pipeline and finite-law controls', status='passed',
            phase='Both original tests passed with additional lawful and unlawful finite-constructor instances',
            passed=finite_law_pipeline['passed'], failed=0, total=finite_law_pipeline['total'],
            source=finite_law_pipeline['compiler_source_sha256'], evidence=link('bootstrap/current-schema9-finite-law-pipeline-results.json')))
    if stage_expansion_proof:
        controls = stage_expansion_proof['controls']
        active.append(dict(title='Cinch staging: typed runtime and original array case', status=stage_expansion_proof['status'],
            phase='Execute the generated Nat program and preserve the named refusal in the unchanged array case',
            passed=sum(row['status'] == 'passed' for row in controls), failed=0, total=len(controls),
            source=stage_expansion_proof['compiler_source_sha256']))
    if stage_syntax:
        active.append(dict(title='Cinch staging syntax', status=stage_syntax['status'],
            phase='The complete syntax target checks staged declarations, generation and splices',
            passed=1, failed=0, total=1, source=stage_syntax['compiler_source_sha256']))
    if schema9_cross:
        active.append(dict(title='Schema 9: Native owner to Cinch consumer', status=schema9_cross['status'],
            phase='Cinch inherits the checked default from the Native artifact and its authenticated dependencies',
            passed=int(schema9_cross['status'] == 'passed'), failed=0, total=1,
            source=schema9_cross['compiler_source_sha256']))
    if schema9_provider:
        status = schema9_provider['status']
        active.append(dict(title='Schema 9 library interfaces', status=status, phase='Regenerate and verify the library-only object and its 55 interfaces',
            passed=int(status.startswith('passed')), failed=int(status.startswith('failed')), total=1,
            source=schema9_provider.get('producer_source_sha256')))
    if schema9_chain:
        status = schema9_chain['status']
        phase = 'Stage2 required the missing Data.List.list-length [R] variant from the older development provider' if schema9_pinned_proof else 'Fresh Stage1, Stage2 and Stage3 with the checked development provider'
        active.append(dict(title='Schema 9 development bootstrap', status=status, phase=phase,
            passed=sum(row.get('status') == 'passed' for row in schema9_chain.get('stages', [])),
            failed=int(status.startswith('failed')), total=4, source=schema9_chain.get('compiler_source_sha256')))
    if schema9_pinned_chain:
        status = schema9_pinned_chain['status']
        audited = (pinned_audit.get('status') == 'passed-production-schema9-bootstrap-chain'
                   and pinned_audit.get('compiler_source_sha256') == schema9_pinned_chain.get('compiler_source_sha256'))
        active.append(dict(title='Schema 9 bootstrap with the production pin', status='passed' if audited else status,
            phase='Fixed point, separate interface consumer and changed-source control verified' if audited else status.replace('-', ' '), passed=sum(row.get('status') == 'passed' for row in schema9_pinned_chain.get('stages', [])),
            failed=int(status.startswith('failed')), total=4, source=schema9_pinned_chain.get('compiler_source_sha256')))
    if schema9_ordering_contracts:
        rows = schema9_ordering_contracts.get('rows', [])
        active.append(dict(title='Ordering: compare interfaces from the same source', status=schema9_ordering_contracts['status'],
            phase='Cinch and Rust check the original module with the same dependency closure',
            passed=sum(row['exit_code'] == 0 for row in rows), failed=sum(row['exit_code'] != 0 for row in rows), total=1))
    if family_replay:
        rows = family_replay.get('rows', [])
        active.append(dict(title='Unchanged source-library families: structural arrow candidate',
            status=family_replay['status'], phase=family_replay.get('phase'),
            passed=sum(r['exit_code'] == 0 for r in rows), failed=sum(r['exit_code'] != 0 for r in rows),
            total=4, source=arrow_receipt.get('compiler_source_sha256')))

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
