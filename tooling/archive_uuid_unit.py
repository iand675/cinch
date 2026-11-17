#!/usr/bin/env python3
"""Durably archive one closed successful UUID unit with a hard10MiB compressed scratch limit."""
from __future__ import annotations
import argparse
import hashlib
import json
import os
from pathlib import Path
import resource
import stat
import tarfile
from validate_uuid import ROOT, digest


def sync_directory(path: Path) -> None:
    """Persist directory entries before relying on an archive or removing an original."""
    fd=os.open(path,os.O_RDONLY|os.O_DIRECTORY)
    try:os.fsync(fd)
    finally:os.close(fd)


def save(path: Path, value: dict) -> None:
    """Atomically persist a recoverable index/report and its containing directory."""
    temporary=path.with_name(path.name+'.tmp')
    with temporary.open('w') as stream:
        json.dump(value,stream,indent=2);stream.write('\n');stream.flush();os.fsync(stream.fileno())
    os.replace(temporary,path);sync_directory(path.parent)


def main() -> None:
    """Check exact results and all original/archive member bytes before unlinking only closed outputs."""
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--report',type=Path,required=True)
    parser.add_argument('--cpu',type=int,required=True)
    args=parser.parse_args();os.sched_setaffinity(0,{args.cpu})
    resource.setrlimit(resource.RLIMIT_FSIZE,(10*1024*1024,10*1024*1024))
    report=json.loads(args.report.read_text())
    assert report['status']=='passed' and 'closed_archive' not in report
    epoch=(report['compiler_sha256'],report['runtime_sha256'])
    rust=('330adb4d77f8bcbfc26f246269280f391c3a10b6d3a40771b5f6243f6a8969b2','d91ce012395a99e1148232d20ec316979cd7d0cb13647a3283ba4820192c0abf')
    native=('2f07707e0022b55e92bd8c2412962d06d2e4455022261c3e36dfa27e57fb231f','80bd5b546742f8374221ac445ec20b7cfd0df0958d5c39313cde077204d6e141')
    assert epoch in (rust,native)
    if epoch==native:
        assert report['kind']=='strict-historical-Native-pure-UUID-unit' and not report['io_or_mvar_requested']
        assert report['interface_sha256']=='1d22f22c4e4e0831e1176b70b4dd77cfa13c5d6db83869a596f64b34cd252256'
        assert report['objects_sha256']=='22ab850df180636adac27eafb7a789dfd89009000c572b194c05d0a192bc5624'
        assert report['provider_object_sha256']=='46a7e57f467dd36135238903910feee82998526cbb3edaa94580a2a40ba47e64'
    assert len(report['runs'])==2 and [r['poisoned_free'] for r in report['runs']]==[False,True]
    for run in report['runs']:
        assert run['exit']==0 and run['stdout']=='' and run['stack']=='8M'
        assert run['heap']==('256M' if run['poisoned_free'] else '32M')
        assert 'cinch-leak: live=0 immortal=0 leaked=0' in run['stderr']
    executable=Path(report['runs'][0]['command'][0]).resolve();llvm=executable.with_suffix('.ll')
    assert executable.parent.parent==ROOT/'build' and executable.parent.name.startswith('uuid-') and executable.name=='consumer'
    assert digest(executable)==report['executable_sha256'] and digest(llvm)==report['llvm_sha256']
    for module,expected in report['sources'].items():assert digest(executable.parent/(module if module.endswith('.cinch') else module+'.cinch'))==expected
    files=[executable,llvm]+sorted(p for p in (executable.parent/'build').rglob('*') if p.is_file())
    assert all(not p.is_symlink() and p.is_relative_to(executable.parent) for p in files)
    members={str(p.relative_to(ROOT)):{'sha256':digest(p),'bytes':p.stat().st_size,'mode':stat.S_IMODE(p.stat().st_mode)} for p in files}
    directory=ROOT/'build/uuid-closed-unit-archive';directory.mkdir(parents=True,exist_ok=True)
    key=hashlib.sha256(str(executable).encode()).hexdigest()[:16]
    archive,snapshot=directory/(key+'.tar.gz'),directory/(key+'.result.json')
    assert not archive.exists() and not snapshot.exists()
    with snapshot.open('xb') as stream:stream.write(args.report.read_bytes());stream.flush();os.fsync(stream.fileno())
    temporary=archive.with_suffix('.gz.tmp')
    with temporary.open('xb') as stream:
        with tarfile.open(fileobj=stream,mode='w:gz',compresslevel=1) as packed:
            for p in files:packed.add(p,arcname=str(p.relative_to(ROOT)),recursive=False)
        stream.flush();os.fsync(stream.fileno())
    assert temporary.stat().st_size<=10*1024*1024
    with tarfile.open(temporary,'r:gz') as packed:
        assert len(packed.getmembers())==len(members)
        for member in packed.getmembers():
            expected=members[member.name];assert member.isfile() and member.mode==expected['mode']
            checksum,size=hashlib.sha256(),0;stream=packed.extractfile(member)
            while block:=stream.read(1048576):checksum.update(block);size+=len(block)
            assert (checksum.hexdigest(),size)==(expected['sha256'],expected['bytes'])
    os.replace(temporary,archive);sync_directory(directory)
    index_path=directory/'index.json';index=json.loads(index_path.read_text()) if index_path.exists() else {'kind':'verified-closed-UUID-functional-units','entries':[]}
    entry={'report':str(args.report.resolve()),'snapshot':str(snapshot.relative_to(ROOT)),'snapshot_sha256':digest(snapshot),
           'archive':str(archive.relative_to(ROOT)),'archive_sha256':digest(archive),'archive_bytes':archive.stat().st_size,
           'members':members,'originals_removed':False}
    index['entries'].append(entry);save(index_path,index)
    report['closed_archive']={'index':str(index_path.relative_to(ROOT)),**entry};save(args.report.resolve(),report)
    for p in files:assert digest(p)==members[str(p.relative_to(ROOT))]['sha256'];p.unlink()
    for parent in {p.parent for p in files}:sync_directory(parent)
    entry['originals_removed']=True;save(index_path,index);report['closed_archive']['originals_removed']=True;save(args.report.resolve(),report)
    print(json.dumps({'original_bytes':sum(e['bytes'] for e in members.values()),'archive_bytes':entry['archive_bytes'],
                      'index':str(index_path),'index_sha256':digest(index_path),'free_bytes':os.statvfs(ROOT).f_bavail*os.statvfs(ROOT).f_frsize}))

if __name__=='__main__':main()
