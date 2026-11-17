#!/usr/bin/env python3
"""Compile bounded public literal-lookup consumers from unchanged pinned-SDK observations and compare every returned offset."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]
sys.path.insert(0,str(ROOT/'tooling'))
from bench_stdlib import source_closure

HEADER='''-- | Compare every public suffix lookup position with independently recorded reference output.
module Main where
  import Data.Index.SuffixArray
  import Data.Bool
  import Data.Bytes
  import Data.List
  import Data.Nat
  import Data.Option
  import Data.Result
  import Data.String
  import Effect.Process

  -- | Compare complete ordered positions without a fingerprint or recursive allocation on unwind.
  same :: List Nat -> List Nat -> Bool
  same first second = case first of
    Nil -> case second of Nil -> True; other -> False
    Cons a rest -> case second of
      Nil -> False
      Cons b tail -> if a == b then same rest tail else False
'''


def digest(path: Path) -> str:
    """Bind each result to the exact compiler, source epoch and unchanged oracle bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def numbers(values: list[int]) -> str:
    """Render explicit expected natural offsets without relying on a parser for printed results."""
    return '['+','.join(map(str,values))+']'


def consumer(rows: list[dict]) -> str:
    """Build each index once and report the first differing query as an exact bounded exit code."""
    source=HEADER
    for i,row in enumerate(rows):
        source+=f'\n  -- | Check all reference literal limits for corpus input {i}.\n  case{i} :: Unit -> Int\n  case{i} unit = case suffix-index (bytes-from-list {numbers(list(bytes.fromhex(row["data"])))}) of\n    Err failure -> 100\n    Ok index ->\n'
        for j,query in enumerate(row['queries']):
            limit='AllSuffixMatches' if query['limit']<0 else f'(AtMostSuffixMatches {query["limit"]})'
            source+=f'      let q{j} = same (suffix-index-lookup index (bytes-from-list {numbers(list(bytes.fromhex(query["needle"])))}) {limit}) {numbers(query["offsets"] or [])} in\n'
        source+='      '+''.join(f'if not q{j} then {j+1} else ' for j in range(len(row['queries'])))+'0\n'
    dispatch='101'
    for i in reversed(range(len(rows))):dispatch=f'if number == "{i}" then case{i} () else {dispatch}'
    source+=f'''\n  -- | Select one bounded independent input, keeping every poison quarantine separate.
  main :: Unit ->{{IO r}} Int
  main unit = with (io = newIO ()) in case process-args () of
    Ok (Cons number Nil) -> {dispatch}
    other -> 102
'''
    return source


def main() -> None:
    """Stop on the first real compile/output failure and retain all diagnostic artifacts plain."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--compiler',type=Path,required=True)
    parser.add_argument('--runtime',type=Path,required=True)
    parser.add_argument('--start',type=int,required=True)
    parser.add_argument('--count',type=int,default=8)
    parser.add_argument('--opt-level',choices=['0','2'],default='0')
    parser.add_argument('--artifacts',type=Path,required=True)
    args=parser.parse_args();work=args.artifacts.resolve();report=work.with_suffix('.json')
    assert work.parent==ROOT/'build' and work.name.startswith('suffixarray-v1-runtime-')
    assert not work.exists() and not report.exists() and 0<args.count<=8
    before=shutil.disk_usage(ROOT).free;assert before>=350*1024*1024
    assert digest(args.compiler)=='c2ae24e64f7999060dce8610618f316710f6300ca29e89b9335056936344a86e'
    assert digest(args.runtime)=='d91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf'
    vectors=json.loads((HERE/'vectors.json').read_text());rows=vectors['cases'][args.start:args.start+args.count];assert rows
    for row in rows:
        data=bytes.fromhex(row['data'])
        for query in row['queries']:
            needle=bytes.fromhex(query['needle']);matches=[i for i in (row['suffix_order'] or []) if needle and data[i:].startswith(needle)]
            expected=matches if query['limit']<0 else matches[:query['limit']]
            assert (query['offsets'] or [])==expected,'independent suffix-order observation disagreement'
    work.mkdir();entry=work/'Main.cinch';entry.write_text(consumer(rows))
    dependencies=source_closure(entry.read_text(),{'Data.Index.SuffixArray':HERE/'src/Data.Index.SuffixArray.cinch'})
    for name,path in dependencies.items():shutil.copyfile(path,work/(name+'.cinch'))
    executable=work/'consumer'
    command=[str(args.compiler),'build',str(entry),'-o',str(executable),'--opt-level',args.opt_level,'--runtime-archive',str(args.runtime)]
    r=dict(scope='Bounded exact public suffix construction and overlapping literal outputs; no performance claim',status='building',command=command,source_sha256=digest(HERE/'src/Data.Index.SuffixArray.cinch'),entry_sha256=digest(entry),vectors_sha256=digest(HERE/'vectors.json'),compiler_sha256=digest(args.compiler),runtime_sha256=digest(args.runtime),dependencies={n:digest(p) for n,p in dependencies.items()},start=args.start,count=len(rows),literal_observations=sum(len(x['queries']) for x in rows),compiled_rc_debug=False,profiles=[],before_free_bytes=before)
    report.write_text(json.dumps(r,indent=2)+'\n');start=time.monotonic()
    try:
        result=subprocess.run(command,capture_output=True,text=True);(work/'compile.log').write_text(result.stdout+result.stderr)
        assert result.returncode==0,result.stdout+result.stderr
        r.update(executable_sha256=digest(executable),llvm_sha256=digest(work/'consumer.ll'),status='running');report.write_text(json.dumps(r,indent=2)+'\n')
        for poison in [False,True]:
            for i in range(len(rows)):
                env={**os.environ,'CINCH_RC_DEBUG':str(int(poison)),'CINCH_HEAP_LIMIT':'256M' if poison else '32M','CINCH_STACK_SIZE':'8M','CINCH_LEAK_CHECK':'1'}
                result=subprocess.run([str(executable),str(i)],env=env,capture_output=True,text=True)
                (work/f'run-{poison}-{i}.stdout').write_text(result.stdout);(work/f'run-{poison}-{i}.stderr').write_text(result.stderr)
                assert result.returncode==0 and not result.stdout and 'leaked=0' in result.stderr,(args.start+i,result.returncode,result.stdout,result.stderr)
                r['profiles'].append(dict(poison=poison,corpus_input=args.start+i,zero_leaks=True));report.write_text(json.dumps(r,indent=2)+'\n')
        r['status']='passed'
    except Exception as error:r.update(status='failed',diagnostics=repr(error));raise
    finally:
        r.update(seconds=time.monotonic()-start,after_free_bytes=shutil.disk_usage(ROOT).free,own_output_bytes=sum(p.stat().st_size for p in work.rglob('*') if p.is_file()));report.write_text(json.dumps(r,indent=2)+'\n');print(json.dumps(r))
    assert r['own_output_bytes']<=30*1024*1024 and r['after_free_bytes']>=250*1024*1024


if __name__=='__main__':main()
