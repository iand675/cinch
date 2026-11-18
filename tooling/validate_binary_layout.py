#!/usr/bin/env python3
"""Execute independent typed-binary fixtures through either frozen frontend."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
from bench_stdlib import source_closure

ROOT = Path(__file__).resolve().parents[1]
KINDS = ['bool','u8','i8','u16','i16','u32','i32','u64','i64','f32','f64','c64','c128']
ORDERS = ['little','big','native']


def checks(group: str, rows: list[dict]) -> list[str]:
    """Render public observations from independently generated expected bytes."""
    out = []
    for row in rows:
        k = KINDS.index(row['kind']) if row.get('kind') in KINDS else 0
        o = ORDERS.index(row['order']) if 'order' in row else 0
        if group == 'scalars':
            out.append(f'scalar-case {k} {o} {row["bits"]} {row["imaginary"]} {row["size"]} "{row["hex"]}"')
        elif group == 'arrays':
            out.append(f'array-case {k} {o} {row["count"]} {row["size"]} "{row["hex"]}" "{row["sha256"]}"')
        elif group == 'records':
            args = [str(o), 'True' if row['active'] else 'False'] + [str(row[x]) for x in ['code','count_bits','single_bits','double_bits','real_bits','imaginary_bits']]
            args += ['[' + ','.join(map(str,row['samples'])) + ']', f'"{row["hex"]}"']
            out.append('record-case ' + ' '.join(args))
        elif group == 'varints':
            out.append(f'varint-case {row["bits"]} "{row["unsigned"]}" "{row["signed"]}"')
        elif group == 'prefixes':
            tags = {'':0,'incomplete':1,'overflow':2,'eof':1,'truncated':2}
            out.append(f'prefix-case "{row["input"]}" {tags[row["kind"]]} {row["count"]} {row["value"]} {tags[row["stream_kind"]]} {row["stream_count"]} {row["stream_value"]} "{row["remaining"]}"')
    return out


def consumer(expressions: list[str], extra: str = '') -> str:
    """Keep the source batches bounded while retaining all runtime observations."""
    source = (ROOT/'compiler/tests/fixtures/binary-layout-consumers.cinchpart').read_text() + '\n' + extra
    names = []
    for i in range(0,len(expressions),96):
        name = f'checks{i//96}'
        names.append(name + ' ()')
        source += f'\n  -- | Observe independent fixture batch {i//96}.\n  {name} :: forall r. using (io :: Io r) => Unit ->{{IO r}} Bool\n  {name} unit = all [\n' + ',\n'.join(f'    ({x})' for x in expressions[i:i+96]) + '\n    ]\n'
    return source + '\n  -- | Observe the complete executable fixture result.\n  main :: Unit ->{IO r} Int\n  main unit = with (io = newIO ()) in if ' + (' && '.join(names) or 'True') + ' then 0 else 1\n'


def run(command: list[str], **kwargs) -> subprocess.CompletedProcess:
    """Stop dependent steps on failure and retain all available diagnostics."""
    result = subprocess.run(command,capture_output=True,text=True,**kwargs)
    if result.returncode:
        raise RuntimeError(f'{command}\n{result.stdout}\n{result.stderr}')
    return result


def main() -> None:
    """Compile source-distributed modules and check normal and poisoned ownership runs."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--native-interfaces',type=Path)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--groups',nargs='+',default=['scalars','arrays','records','varints','prefixes','contracts','streams','stress','stream-stress'])
    parser.add_argument('--first',type=int,default=0)
    parser.add_argument('--limit',type=int)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args()
    fixture=json.loads((ROOT/'compiler/tests/fixtures/binary-layout-vectors.json').read_text())
    actual_compiler=args.compiler.parent/'cinch' if args.native_interfaces else args.compiler
    report={'compiler_sha256':hashlib.sha256(actual_compiler.read_bytes()).hexdigest(),'frontend':str(args.compiler.resolve()),'frontend_sha256':hashlib.sha256(args.compiler.read_bytes()).hexdigest(),'sources':{},'results':[]}
    for group in args.groups:
        work=args.artifacts/group;work.mkdir(parents=True,exist_ok=True)
        extra=''
        if group in ['contracts','streams','stress','stream-stress','records','varints','prefixes']:
            extra=(ROOT/'compiler/tests/fixtures/binary-layout-contracts.cinchpart').read_text()
        rows=fixture.get(group,[None])[args.first:]
        if args.limit is not None:rows=rows[:args.limit]
        expressions=[group+'-case ()'] if group in ['contracts','streams','stress','stream-stress'] else checks(group,rows)
        source=consumer(expressions,extra)
        (work/'Main.cinch').write_text(source)
        deps=source_closure(source)
        for name,path in deps.items():
            shutil.copyfile(path,work/(name+'.cinch'))
            report['sources'][name]=hashlib.sha256(path.read_bytes()).hexdigest()
        exe=(work/'consumer').resolve()
        if args.native_interfaces:
            bundle=work/'Bundle.cinch'
            bundle.write_text('\n'.join(path.read_text() for path in reversed(list(deps.values())))+'\n'+source)
            command=[sys.executable,str(args.compiler.resolve()),'--rc-debug','--compile-only','--interfaces',str(args.native_interfaces.resolve()),str(bundle.resolve()),str(exe.with_suffix('.ll')),str(exe),str(exe.with_suffix('.link.txt'))]
        else:command=[str(args.compiler.resolve()),'build',str((work/'Main.cinch').resolve()),'-o',str(exe),'--opt-level','2','--rc-debug']
        try:run(command)
        except RuntimeError as e:
            diag=exe.with_suffix('.ll.diagnostics')
            raise RuntimeError(str(e)+'\n'+(diag.read_text() if diag.exists() else '')) from e
        observed=[]
        for debug in [False,True]:
            result=run([str(exe)],env={**os.environ,'CINCH_RC_DEBUG':'1' if debug else '0','CINCH_LEAK_CHECK':'1','CINCH_HEAP_LIMIT':'256M' if debug else '32M','CINCH_STACK_SIZE':'8M'})
            assert 'leaked=0' in result.stderr,result.stderr
            observed.append({'debug':debug,'zero_leaks':True})
        report['results'].append({'group':group,'cases':len(rows),'runs':observed})
        print(f'{group}: {len(rows)} cases, both ownership modes passed',flush=True)
    args.output.parent.mkdir(parents=True,exist_ok=True)
    args.output.write_text(json.dumps(report,indent=2)+'\n')

if __name__=='__main__':main()
