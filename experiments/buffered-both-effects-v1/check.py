#!/usr/bin/env python3
"""Check isolated buffered IO+Mut source copies with exact frozen compiler identities and no production edits."""
from pathlib import Path
import argparse,hashlib,json,os,re,resource,subprocess,time

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
RUST=Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen')
NATIVE=Path('/workspace/dasl-stdlib-string-combined/build/combined-stage3-frozen')


def digest(path):
    """Record exact source, compiler, interface, runtime and diagnostic bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def effect_refusal(diagnostic):
    """Recognize explicit effect-row disagreement in either frontend, never an unrelated parser/import refusal."""
    return 'Mut' in diagnostic and (
        re.search(r'effect rows? (?:do not match|mismatch)',diagnostic,re.I) is not None
        or re.search(r'type mismatch: expected \{\(IO[^\n]*found \{\(IO[^\n]*\(Mut',diagnostic) is not None
    )


def host_stack():
    """Keep the compiler-host limit explicit, independent of future fixed consumer profiles."""
    resource.setrlimit(resource.RLIMIT_STACK,(64*1024*1024,resource.getrlimit(resource.RLIMIT_STACK)[1]))


def source_closure(project,source,embedded,dependency_snapshot):
    """Resolve exact experiment overrides and an optional qualified multipart closure without shadowing the selected base family."""
    overrides={path.name[:-6]:path for path in (HERE/'sources').glob('*.cinch')}
    overrides['Experiment.Buffered.Multipart']=HERE/'MultipartAdapter.cinch'
    pending=re.findall(r'^\s*import\s+([\w.]+)',source.read_text(),re.M);found=set();paths=[]
    while pending:
        name=pending.pop()
        if name in embedded or name in found:continue
        found.add(name);original=overrides.get(name)
        if original is None and dependency_snapshot is not None:
            candidate=dependency_snapshot/(name+'.cinch')
            if candidate.exists():original=candidate
        if original is None:original=ROOT/'compiler/prelude'/(name.replace('.','/')+'.cinch')
        target=project/(name+'.cinch');target.write_bytes(original.read_bytes());paths.append(target)
        pending.extend(re.findall(r'^\s*import\s+([\w.]+)',target.read_text(),re.M))
    return paths+[source]


def main():
    """Freeze the complete experimental closure before one source-only check and preserve a failed verdict too."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--frontend',choices=['rust','native'],required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--source',type=Path,default=HERE/'Main.cinch')
    parser.add_argument('--native-snapshot',type=Path,default=NATIVE)
    parser.add_argument('--dependency-snapshot',type=Path)
    parser.add_argument('--expect-refusal',action='store_true')
    parser.add_argument('--cpu',type=int,default=2)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    native=args.native_snapshot.resolve();snapshot=RUST if args.frontend=='rust' else native
    compiler=snapshot/'cinch';runtime=snapshot/'libcinchruntime.a';provenance_path=snapshot/('provenance.json' if args.frontend=='rust' else 'snapshot.json')
    if not provenance_path.exists():provenance_path=snapshot/'provenance.json'
    provenance=json.loads(provenance_path.read_text())
    if 'files' in provenance:
        for name in ['cinch','libcinchruntime.a','base-interfaces.json','objects.json']:assert digest(snapshot/name)==provenance['files'][name]['sha256']
    else:
        assert digest(compiler)==provenance['compiler_sha256']
        assert digest(runtime)==provenance['runtime_sha256']
    interfaces=native/'base-interfaces.json'
    artifacts=args.artifacts.resolve();artifacts.mkdir(parents=True,exist_ok=True)
    project=artifacts/'project';project.mkdir(exist_ok=True)
    source=project/'Main.cinch';source.write_bytes(args.source.read_bytes())
    paths=source_closure(project,source,{row[0] for row in json.loads(interfaces.read_text())['interfaces']},args.dependency_snapshot)
    prefix=artifacts/'check-output'
    if args.frontend=='rust':command=[str(compiler),'check',str(source)]
    else:
        bundle=artifacts/'bundle.cinch';bundle.write_text('\n\n'.join(path.read_text() for path in sorted(paths)))
        command=[str(compiler),'--check','--interfaces',str(interfaces),str(bundle),str(prefix)]
    before=time.monotonic()
    result=subprocess.run(command,cwd=project,capture_output=True,text=True,preexec_fn=host_stack,env={**os.environ,'CINCH_HEAP_LIMIT':'2G','CINCH_RC_DEBUG':'0','CINCH_LEAK_CHECK':'1'})
    (artifacts/'check.stdout').write_text(result.stdout);(artifacts/'check.stderr').write_text(result.stderr)
    sidecar=prefix.with_name(prefix.name+'.diagnostics')
    diagnostic=(sidecar.read_text() if sidecar.exists() else '')+result.stdout+result.stderr
    report={'status':'source/type/proof check only; no runtime or promotion claim','frontend':args.frontend,'compiler_sha256':digest(compiler),'runtime_sha256':digest(runtime),'provenance_sha256':digest(provenance_path),'provenance':provenance,'interfaces_sha256':digest(interfaces),'dependency_snapshot':str(args.dependency_snapshot) if args.dependency_snapshot else None,'source_shadow_used':False,'sources':{p.name:digest(p) for p in paths},'signature_manifest_sha256':digest(HERE/'signature-impact.json'),'command':command,'returncode':result.returncode,'diagnostic':diagnostic,'seconds':time.monotonic()-before,'compiler_host_heap':'2G','compiler_host_stack_mib':64,'cpu':args.cpu,'expected_effect_refusal':args.expect_refusal}
    (artifacts/'receipt.json').write_text(json.dumps(report,indent=2)+'\n')
    print(diagnostic or 'Isolated '+args.frontend+' both-effect source check passed',flush=True)
    if args.expect_refusal:
        if result.returncode==0 or not effect_refusal(diagnostic):raise RuntimeError('expected genuine mutation-row refusal was not observed')
        return
    raise SystemExit(result.returncode)


if __name__=='__main__':main()
