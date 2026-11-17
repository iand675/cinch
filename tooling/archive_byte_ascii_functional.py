#!/usr/bin/env python3
"""Losslessly archive completed successful ASCII/Fields consumer executables and LLVM, with durable recovery evidence."""
from __future__ import annotations
import argparse
import gzip
import hashlib
import json
import os
from pathlib import Path
import shutil
import stat

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    """Hash an artifact in bounded chunks before recording or reclaiming its storage."""
    result = hashlib.sha256()
    with path.open('rb') as stream:
        for block in iter(lambda: stream.read(1024*1024),b''):
            result.update(block)
    return result.hexdigest()


def sync_directory(path: Path) -> None:
    """Persist every recovery rename and every original-file unlink in its parent directory."""
    descriptor = os.open(path,os.O_DIRECTORY)
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def save(index: Path, state: dict) -> None:
    """Atomically fsync a complete recoverable manifest before an original can be unlinked."""
    temporary = index.with_suffix('.tmp')
    with temporary.open('w') as stream:
        json.dump(state,stream,indent=2)
        stream.write('\n')
        stream.flush()
        os.fsync(stream.fileno())
    os.replace(temporary,index)
    sync_directory(index.parent)


def verify(archive: Path, entry: dict) -> None:
    """Verify compressed bytes, full decompression, exact recovered size and original content identity."""
    assert digest(archive) == entry['compressed_sha256']
    result = hashlib.sha256()
    size = 0
    with gzip.open(archive,'rb') as stream:
        for block in iter(lambda: stream.read(1024*1024),b''):
            result.update(block)
            size += len(block)
    assert size == entry['size']
    assert result.hexdigest() == entry['sha256'] == entry['decompressed_sha256']


def main() -> None:
    """Keep sources, all frozen/provider/runtime/benchmark inputs and all failure evidence outside this reclaim scope."""
    parser = argparse.ArgumentParser(description=__doc__)
    action = parser.add_mutually_exclusive_group(required=True)
    action.add_argument('--report',type=Path,nargs='+')
    action.add_argument('--restore',type=Path,nargs='+')
    parser.add_argument('--index',type=Path,default=ROOT/'build/byte-ascii-functional-recovery/index.json')
    args = parser.parse_args()
    index = args.index.resolve()
    assert index.is_relative_to(ROOT/'build')
    index.parent.mkdir(parents=True,exist_ok=True)
    archives = index.parent/'files'
    archives.mkdir(exist_ok=True)
    sync_directory(index.parent)
    sync_directory(index.parent.parent)
    state = json.loads(index.read_text()) if index.exists() else dict(schema=1,files=[])
    state['scope'] = 'closed successful ASCII/Fields functional ELF/LLVM only'
    before = shutil.disk_usage(ROOT).free
    count = 0
    reclaimed = 0
    if args.restore:
        for requested in args.restore:
            original = requested.resolve()
            assert original.is_relative_to(ROOT/'build')
            entry = next(value for value in state['files'] if value['path']==str(original.relative_to(ROOT)))
            verify(ROOT/entry['archive'],entry)
            if original.exists():
                assert digest(original)==entry['sha256'] and stat.S_IMODE(original.stat().st_mode)==entry['mode']
                continue
            assert shutil.disk_usage(ROOT).free>=entry['size']+8*1024*1024
            temporary = original.with_name(original.name+'.recovery-partial')
            assert not temporary.exists()
            with temporary.open('wb') as target:
                with gzip.open(ROOT/entry['archive'],'rb') as source:
                    shutil.copyfileobj(source,target,1024*1024)
                target.flush()
                os.fchmod(target.fileno(),entry['mode'])
                os.fsync(target.fileno())
            assert digest(temporary)==entry['sha256'] and temporary.stat().st_size==entry['size']
            os.replace(temporary,original)
            sync_directory(original.parent)
            entry['original_unlinked']=False
            save(index,state)
            count += 1
        print(json.dumps(dict(files_restored=count,before_free_bytes=before,after_free_bytes=shutil.disk_usage(ROOT).free,index=str(index))))
        return
    for report in args.report:
        report = report.resolve()
        assert report.parent == ROOT/'build' and (report.name.startswith('byte-ascii-run-') or (report.name.startswith('byte-fields-') and ('-runtime-' in report.name or report.name == 'byte-fields-first-native-v4-owners-o0.json')))
        result = json.loads(report.read_text())
        assert result['status'] == 'passed' and result['profiles'] and all(value['zero_leaks'] for value in result['profiles'])
        directory = report.with_suffix('')
        for name,key in [('consumer','executable_sha256'),('consumer.ll','llvm_sha256')]:
            original = directory/name
            path = str(original.relative_to(ROOT))
            previous = next((entry for entry in state['files'] if entry['path']==path),None)
            if previous:
                assert previous['sha256'] == result[key]
                verify(ROOT/previous['archive'],previous)
                entry = previous
            else:
                metadata = original.lstat()
                assert stat.S_ISREG(metadata.st_mode) and digest(original) == result[key]
                archive = archives/(result[key]+'.gz')
                if archive.exists():
                    equivalent = next(value for value in state['files'] if value['sha256']==result[key])
                    assert equivalent['size']==metadata.st_size
                    verify(archive,equivalent)
                else:
                    temporary = archive.with_suffix('.partial')
                    assert not temporary.exists()
                    with temporary.open('wb') as target:
                        with gzip.GzipFile(fileobj=target,mode='wb',filename='',mtime=0,compresslevel=6) as output:
                            with original.open('rb') as source:
                                shutil.copyfileobj(source,output,1024*1024)
                        target.flush()
                        os.fsync(target.fileno())
                    temporary.chmod(0o444)
                    with temporary.open('rb') as stream:
                        os.fsync(stream.fileno())
                    os.replace(temporary,archive)
                    sync_directory(archives)
                entry = dict(path=path,size=metadata.st_size,mode=stat.S_IMODE(metadata.st_mode),sha256=result[key],
                             archive=str(archive.relative_to(ROOT)),compressed_sha256=digest(archive),decompressed_sha256=result[key],
                             compressed_bytes=archive.stat().st_size,receipt=str(report.relative_to(ROOT)),receipt_sha256=digest(report),
                             original_unlinked=False)
                verify(archive,entry)
                state['files'].append(entry)
            save(index,state)
            if original.exists():
                metadata = original.lstat()
                assert stat.S_ISREG(metadata.st_mode) and metadata.st_size == entry['size']
                assert stat.S_IMODE(metadata.st_mode) == entry['mode'] and digest(original) == entry['sha256']
                original.unlink()
                sync_directory(original.parent)
                count += 1
                reclaimed += entry['size']-entry['compressed_bytes']
            entry['original_unlinked'] = True
            save(index,state)
    print(json.dumps(dict(files_reclaimed=count,logical_net_bytes=reclaimed,before_free_bytes=before,after_free_bytes=shutil.disk_usage(ROOT).free,index=str(index))))


if __name__ == '__main__':
    main()
