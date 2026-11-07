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
        same = front.get('compiler_source_sha256') == compiler
        state = {'passed':'verified', 'failed':'blocked', 'running':'running'}.get(old.get('status'), 'pending') if same and old else 'recheck' if old and old.get('status') != 'pending' else 'pending'
        front_rows.append(item('stage3:'+name, name, state,
            'Requires the latest genuinely self-built Stage3. Development-compiler checks are listed separately.',
            'Run the complete original target through the matching Stage3; separately classify direct Rust API assertions.',
            link('compiler/tests/'+name+'.rs'), observed=observed(old, front.get('compiler_source_sha256'), Path(cfg['selected']).parent),
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

    additional = read(cfg.get('sourceClassDefaultApiReceipt'))
    if additional:
        group = next(r for r in additional['groups'] if r['name'] == 'class_defaults')
        row = item('additional:class_defaults', 'class_defaults / actual Cinch frontend', 'recheck',
            'All seven original/new default tests remain. Private75fb passes3/7, improved from1/7: both original strict-value runtime/build tests pass. Three exact diagnostic/source assertions and the refinement client remain failures.',
            'Pass every original default source, runtime, identity and diagnostic/path assertion through the final matching Cinch-built compiler; classify the11 direct Rust annotation checks separately.',
            link('bootstrap/current-source-class-default-self-values-api-results.json'),
            observed=dict(status='failed',source=additional['compiler_source_sha256'],cases=cases(Path(cfg.get('artifactRoot', BASE))/'source-class-default-interface-api-2/class_defaults.log')))
        suites.append(suite('additional','Additional frontend coverage','targets',[row],
            'Added by the coverage audit. All original105 Stage3 targets remain separately required; direct Rust calls do not grant Cinch frontend credit.'))

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
    default_runtime=read(cfg.get('sourceClassDefaultRuntime'))
    strict_runtime=read(cfg.get('sourceClassDefaultSelfValuesRuntimeReceipt'))
    default_api=read(cfg.get('sourceClassDefaultApi'))
    default_full=read(cfg.get('sourceClassDefaultFullCompiler'))
    coverage_state=read(cfg.get('inferredCaseCoverageValidation'))
    coverage_runtime=read(cfg.get('inferredCaseCoverageRuntimeReceipt'))
    function_defaults=read(cfg.get('functionDefaultApiReceipt'))
    default_detail='Latest75fb passes all seven complete default/dependent component groups (nine tests), all nine full compiler/object tests and both original strict-value runtime/build tests. Original API/default replay:16/21 tests pass; source-Prelude, three diagnostic assertions and the refinement runtime client remain failures. Imported checked default transport and matching final bootstrap remain required.'
    gaps=[
        item('gap:prelude','Replace source copies of library modules safely','blocked'if 'interface_prelude_overrides'in failed_api else'pending',
             'The current interface/API run fails when source Data.Core is supplied alongside its cached interface. All four original families remain required.',
             'Core, Ordering, Category and Algebra replacements pass with fresh checked provider hashes, valid consumer chains and no Prelude back-edge.',link('bootstrap/current-source-prelude-counterexample.json')),
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
             f'{len(set(native_names)-set(front_names))} Rust targets are outside the preserved105-target Stage3 matrix. Ten target-level reviews identify two Native harness roles and eight source/mixed targets needing complementary Cinch coverage;120 targets and per-assertion work remain. The whole original two-test fixity replay now passes through actual development2866 with only dispatch changed, exact interface assertions, exit9 and zero leaks. Final matching Stage3 execution remains required.',
             'Review all remaining targets and assertions, retain direct Rust API roles explicitly, and add every missing portable behavior case to final Stage3 execution.',link('bootstrap/current-outside-frontend-target-review.json')),
        item('gap:final-audit','Complete the requirement-by-requirement final audit','pending','A green subset, source check or earlier fixed point cannot close the full goal.',
             'Every acceptance item, named artifact, invariant and complete matching suite has direct current evidence; all failures and unclassified coverage are closed.',link('design/self-hosting.md#acceptance'))]

    active=[]
    next_chain = read(cfg.get('partialBooleanBootstrapChain'))
    if next_chain:
        status = next_chain.get('status', '')
        active.append(dict(
            title='Fresh Stage0 through Stage3 rebuild',
            status='running' if status.startswith(('building-', 'checking-')) else status,
            phase='Waiting for the compiler/object tests, the own-source check, and the nine component targets'
                  if status == 'waiting-matching-development-gates' else status.replace('-', ' '),
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
        ('constructorPublicDisplay','Constructor display: interface and nominal tests',2,'compile-'),
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
            receipt=read(cfg.get(receipt_key)) if receipt_key else annotation_receipt if key.startswith('nativeDefaultAnnotation') else default_receipt
            audited=bool(receipt) and receipt.get('compiler_source_sha256')==state.get('compiler_source_sha256') and (receipt.get('source')==state.get('source') or receipt.get('whole_groups')==total)
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
