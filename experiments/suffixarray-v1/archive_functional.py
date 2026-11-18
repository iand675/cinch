#!/usr/bin/env python3
"""Recover only report-linked successful suffix-index consumer ELF/LLVM, preserving complete durable restoration metadata."""
from __future__ import annotations
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import shutil
import stat

ROOT=Path(__file__).resolve().parents[2]


def digest(path: Path) -> str:
    """Hash every byte with bounded scratch storage."""
    h=hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda:stream.read(1024*1024),b''):h.update(chunk)
    return h.hexdigest()


def sync_directory(path: Path) -> None:
    """Make each index rename and original unlink durable in its own parent."""
    fd=os.open(path,os.O_DIRECTORY)
    try:os.fsync(fd)
    finally:os.close(fd)


def save(path: Path, state: dict) -> None:
    """Persist a complete atomic recovery manifest before removing any original."""
    temporary=path.with_suffix('.partial')
    with temporary.open('w') as stream:
        json.dump(state,stream,indent=2);stream.write('\n');stream.flush();os.fsync(stream.fileno())
    os.replace(temporary,path);sync_directory(path.parent)


def verify(archive: Path, entry: dict) -> None:
    """Check compressed identity and full decoded bytes and length before accepting recovery."""
    assert digest(archive)==entry['compressed_sha256']
    h=hashlib.sha256();size=0
    with gzip.open(archive,'rb') as stream:
        for chunk in iter(lambda:stream.read(1024*1024),b''):h.update(chunk);size+=len(chunk)
    assert size==entry['size'] and h.hexdigest()==entry['sha256']==entry['decompressed_sha256']


def metadata(path: Path) -> dict:
    """Capture exact content size, access mode, owner/group and nanosecond modification time."""
    m=path.lstat();assert stat.S_ISREG(m.st_mode)
    return dict(size=m.st_size,mode=stat.S_IMODE(m.st_mode),uid=m.st_uid,gid=m.st_gid,mtime_ns=m.st_mtime_ns)


def main() -> None:
    """Exclude every source/interface/frozen/provider/runtime/benchmark input and all failed outputs."""
    parser=argparse.ArgumentParser(description=__doc__)
    action=parser.add_mutually_exclusive_group(required=True)
    action.add_argument('--report',type=Path,nargs='+')
    action.add_argument('--restore',type=Path,nargs='+')
    parser.add_argument('--index',type=Path,default=ROOT/'build/suffixarray-v1-functional-recovery/index.json')
    args=parser.parse_args();index=args.index.resolve();assert index.is_relative_to(ROOT/'build')
    assert shutil.disk_usage(ROOT).free>=300*1024*1024
    index.parent.mkdir(parents=True,exist_ok=True);files=index.parent/'files';files.mkdir(exist_ok=True)
    sync_directory(index.parent);sync_directory(index.parent.parent)
    state=json.loads(index.read_text()) if index.exists() else dict(schema=1,scope='Closed successful suffix-index functional consumers only',files=[])
    before=shutil.disk_usage(ROOT).free;count=0
    if args.restore:
        for requested in args.restore:
            original=requested.resolve();assert original.is_relative_to(ROOT/'build')
            entry=next(e for e in state['files'] if e['path']==str(original.relative_to(ROOT)))
            archive=ROOT/entry['archive'];verify(archive,entry)
            if original.exists():assert metadata(original)=={k:entry[k] for k in ['size','mode','uid','gid','mtime_ns']} and digest(original)==entry['sha256'];continue
            assert shutil.disk_usage(ROOT).free-entry['size']>=250*1024*1024
            temporary=original.with_name(original.name+'.recovery-partial');assert not temporary.exists()
            with temporary.open('wb') as out:
                with gzip.open(archive,'rb') as inp:shutil.copyfileobj(inp,out,1024*1024)
                out.flush();os.fchown(out.fileno(),entry['uid'],entry['gid']);os.fchmod(out.fileno(),entry['mode']);os.fsync(out.fileno())
            os.utime(temporary,ns=(entry['mtime_ns'],entry['mtime_ns']))
            assert digest(temporary)==entry['sha256'] and metadata(temporary)=={k:entry[k] for k in ['size','mode','uid','gid','mtime_ns']}
            fd=os.open(temporary,os.O_RDONLY);os.fsync(fd);os.close(fd)
            os.replace(temporary,original);sync_directory(original.parent);entry['original_unlinked']=False;save(index,state);count+=1
    else:
        for requested in args.report:
            report=requested.resolve();assert report.parent==ROOT/'build'
            assert report.name.startswith('suffixarray-v1-runtime-') or report.name=='suffixarray-v1-smoke-o0-v1.json'
            assert not any(word in report.name for word in ['bench','frozen','provider','interface'])
            proof=json.loads(report.read_text());assert proof['status']=='passed' and proof['profiles'] and all(p['zero_leaks'] for p in proof['profiles'])
            for name,key in [('consumer','executable_sha256'),('consumer.ll','llvm_sha256')]:
                original=report.with_suffix('')/name;relative=str(original.relative_to(ROOT))
                entry=next((e for e in state['files'] if e['path']==relative),None)
                if entry:
                    assert entry['sha256']==proof[key] and entry['report_sha256']==digest(report);verify(ROOT/entry['archive'],entry)
                    if not original.exists():continue
                else:
                    entry=dict(path=relative,**metadata(original),sha256=digest(original),report=str(report.relative_to(ROOT)),report_sha256=digest(report))
                    assert entry['sha256']==proof[key] and entry['sha256']!=proof.get('compiler_sha256')
                    archive=files/(entry['sha256']+'.gz');temporary=archive.with_suffix('.partial');assert not temporary.exists()
                    if not archive.exists():
                        with temporary.open('wb') as target:
                            with original.open('rb') as source:
                                with gzip.GzipFile(filename='',mode='wb',fileobj=target,mtime=0) as encoded:
                                    for block in iter(lambda:source.read(1024*1024),b''):
                                        assert target.tell()+len(block)+131072<=10*1024*1024
                                        encoded.write(block)
                            target.flush();os.fsync(target.fileno())
                        assert temporary.stat().st_size<=10*1024*1024
                        os.replace(temporary,archive);sync_directory(files)
                    entry.update(archive=str(archive.relative_to(ROOT)),compressed_size=archive.stat().st_size,compressed_sha256=digest(archive),decompressed_sha256=entry['sha256'],original_unlinked=False)
                    verify(archive,entry);state['files'].append(entry);save(index,state)
                assert metadata(original)=={k:entry[k] for k in ['size','mode','uid','gid','mtime_ns']} and digest(original)==entry['sha256']
                assert shutil.disk_usage(ROOT).free>=250*1024*1024
                original.unlink();sync_directory(original.parent);entry['original_unlinked']=True;save(index,state);count+=1
    print(json.dumps(dict(files=count,before_free_bytes=before,after_free_bytes=shutil.disk_usage(ROOT).free,index=str(index))))


if __name__=='__main__':main()
