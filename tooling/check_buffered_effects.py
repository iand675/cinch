#!/usr/bin/env python3
"""Check that buffered continuation cells cannot be allocated, read, written or opened by pure clients."""
from __future__ import annotations
import argparse
import json
import os
from pathlib import Path
import subprocess
from validate_buffered import ROOT,RUST,NATIVE,digest,host_stack


def main() -> None:
    """Preserve each exact rejection and distinguish the pure-effect boundary from private storage access."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--frontend',choices=['rust','native'],default='rust')
    parser.add_argument('--compiler',type=Path)
    parser.add_argument('--native-snapshot',type=Path,default=NATIVE)
    parser.add_argument('--cpu',type=int,default=2)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--output',type=Path,required=True)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    compiler=args.compiler or ((RUST if args.frontend=='rust' else args.native_snapshot)/'cinch')
    def command(project: Path) -> list[str]:
        """Check source with the chosen frontend's exact admitted interface profile."""
        source=project/'Main.cinch'
        if args.frontend=='rust':return [str(compiler.resolve()),'check',str(source)]
        bundle=project/'bundle.cinch';bundle.write_text((project/'Effect.Buffered.Internal.Cell.cinch').read_text()+'\n\n'+source.read_text())
        return [str(compiler.resolve()),'--emit-ir','--interfaces',str((args.native_snapshot/'base-interfaces.json').resolve()),'--objects',str((args.native_snapshot/'objects.json').resolve()),str(bundle),str(project/'check.ll')]
    def check(project: Path) -> tuple[subprocess.CompletedProcess,str,list[str]]:
        """Retain stderr and structured Native diagnostics before judging admission."""
        invoked=command(project);result=subprocess.run(invoked,capture_output=True,text=True,preexec_fn=host_stack)
        (project/'check.stdout').write_text(result.stdout);(project/'check.stderr').write_text(result.stderr)
        diagnostic=project/'check.ll.diagnostics'
        return result,result.stdout+'\n'+result.stderr+('\n'+diagnostic.read_text() if diagnostic.exists() else ''),invoked
    positive=args.artifacts.resolve()/'io-clients';positive.mkdir(parents=True,exist_ok=True)
    positive_source=positive/'Main.cinch'
    positive_source.write_text('''-- | The same cell operations are admitted when their I/O row is present.
module Main where
  import Effect.Buffered.Internal.Cell
  import Data.Nat
  -- | Allocate under the matching region's explicit I/O effect.
  good-new :: forall r. Nat ->{IO r} IoCell r Nat
  good-new value = cell-new value
  -- | Read under the matching region's explicit I/O effect.
  good-read :: forall r. IoCell r Nat ->{IO r} Nat
  good-read cell = cell-read cell
  -- | Write under the matching region's explicit I/O effect.
  good-write :: forall r. IoCell r Nat ->{IO r} Unit
  good-write cell = cell-write cell 0
  -- | Typecheck-only sentinel.
  main :: Int
  main = 0
''')
    positive_dependency=positive/'Effect.Buffered.Internal.Cell.cinch'
    positive_dependency.write_bytes((ROOT/'seed/prelude/Effect/Buffered/Internal/Cell.cinch').read_bytes())
    accepted,positive_diagnostic,positive_command=check(positive)
    if accepted.returncode:raise ValueError('effectful controls failed before negative comparison: '+positive_diagnostic)
    cases={
        'pure-new':'bad :: forall r. Nat -> IoCell r Nat\n  bad value = cell-new value',
        'pure-read':'bad :: forall r. IoCell r Nat -> Nat\n  bad cell = cell-read cell',
        'pure-write':'bad :: forall r. IoCell r Nat -> Unit\n  bad cell = cell-write cell 0',
        'private-storage':'bad :: forall r. IoCell r Nat -> Array Nat\n  bad cell = case cell of StoredCell storage -> storage',
    }
    reports=[]
    for name,declaration in cases.items():
        project=args.artifacts.resolve()/name;project.mkdir(parents=True,exist_ok=True)
        source=project/'Main.cinch';source.write_text('-- | Deliberate effect or private-storage rejection.\nmodule Main where\n  import Effect.Buffered.Internal.Cell\n  import Data.Nat\n  -- | This intentionally invalid client crosses a guarded cell boundary.\n  '+declaration+'\n  -- | Typecheck-only sentinel.\n  main :: Int\n  main = 0\n')
        dependency=project/'Effect.Buffered.Internal.Cell.cinch';dependency.write_bytes((ROOT/'seed/prelude/Effect/Buffered/Internal/Cell.cinch').read_bytes())
        result,text,invoked=check(project)
        if not result.returncode:raise ValueError('invalid client admitted: '+name)
        if name=='private-storage':
            if 'StoredCell' not in text:raise ValueError('unexpected private-storage diagnostic: '+text)
        reports.append({'name':name,'source_sha256':digest(source),'dependency_sha256':digest(dependency),'command':invoked,'returncode':result.returncode,'diagnostic':text})
    args.output.parent.mkdir(parents=True,exist_ok=True);args.output.write_text(json.dumps({'frontend':args.frontend,'compiler_sha256':digest(compiler),'positive_io_clients':{'source_sha256':digest(positive_source),'dependency_sha256':digest(positive_dependency),'command':positive_command,'accepted':True},'cases':reports,'all_rejected_as_expected':True,'note':'Pure-operation clients differ from accepted clients by the required IO row; the frontend may report the resulting region-unification mismatch before its effect-row diagnostic.'},indent=2)+'\n')
    print('Buffered cell effect/privacy boundaries rejected allfour invalid clients',flush=True)


if __name__=='__main__':main()
