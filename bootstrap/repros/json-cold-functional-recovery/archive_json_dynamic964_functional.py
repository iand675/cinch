#!/usr/bin/env python3
"""Losslessly archive explicitly closed method-stage outputs, preserving all source and current snapshots."""
from pathlib import Path
import gzip
import hashlib
import json
import os
import shutil
import stat
import tarfile
import time
import sys
root=Path('/workspace/dasl-stdlib-json')
stage=sys.argv[1]
assert stage in {'rust-control','rust-basic-edge','native'}
archive=root/f'build/json-cold-dynamic964-functional-{stage}-20261117.tar.gz'
index_path=archive.with_suffix('.index.json')
temporary=archive.with_suffix('.incomplete')
receipt=root/f'bootstrap/stdlib-json-cold-dynamic964-functional-{stage}-archive.json'
tracked_index=root/f'bootstrap/stdlib-json-cold-dynamic964-functional-{stage}-archive-index.json'
assert not any(p.exists() for p in [archive,index_path,temporary,receipt,tracked_index]),'inspect durable state; refuse repeated archive'
os.sched_setaffinity(0,{2})
base=root/'build'
pairs={
 'rust-control':['json-dynamic-final-rust-v2/dynamic-control'],
 'rust-basic-edge':['json-dynamic-final-rust-v2/dynamic','json-dynamic-edge-rust-v2/dynamic-edge'],
 'native':['json-dynamic-native-raw-v1/dynamic','json-dynamic-edge-native-raw-v1/dynamic-edge','json-dynamic-control-native-raw-v2/dynamic-control'],
}
files=sorted(base/name/file for name in pairs[stage] for file in ['consumer','consumer.ll'])
assert len(files)==2*len(pairs[stage]) and all(path.is_file() and not path.is_symlink() for path in files)
receipt_names=['bootstrap/evidence/stdlib/stdlib-json-dynamic-final-rust-v2-results.json','bootstrap/evidence/stdlib/stdlib-json-dynamic-edge-rust-v2-results.json'] if stage.startswith('rust') else ['bootstrap/evidence/stdlib/stdlib-json-dynamic-native-raw-v1-results.json','bootstrap/evidence/stdlib/stdlib-json-dynamic-edge-native-raw-v1-results.json','bootstrap/evidence/stdlib/stdlib-json-dynamic-control-native-raw-v2-results.json']

def digest(path):
    with path.open('rb') as source:return hashlib.file_digest(source,'sha256').hexdigest()
def stream_digest(source):
    h=hashlib.sha256();size=0
    while block:=source.read(1024*1024):h.update(block);size+=len(block)
    return h.hexdigest(),size
def fsync_directory(directory):
    fd=os.open(directory,os.O_RDONLY|os.O_DIRECTORY)
    try:os.fsync(fd)
    finally:os.close(fd)
def durable_json(path,data):
    tmp=path.with_name(path.name+'.pending')
    assert not tmp.exists()
    with tmp.open('w') as out:
        out.write(json.dumps(data,indent=2)+'\n');out.flush();os.fsync(out.fileno())
    os.replace(tmp,path);fsync_directory(path.parent)

entries=[]
for path in files:
    info=path.lstat()
    assert root in path.parents and stat.S_ISREG(info.st_mode) and info.st_nlink==1,str(path)
    entries.append({'path':str(path.relative_to(root)),'source':str(path),'size':info.st_size,'mode':stat.S_IMODE(info.st_mode),'sha256':digest(path),'nlink':info.st_nlink,'allocated_bytes':info.st_blocks*512})
record={'description':'Published964 closed functional ELF/LLVM only; all source/log/reference/docs/frozen compiler/provider/runtime, current snapshots and benchmark inputs remain plain','archive':str(archive),'entries':entries,'original_file_bytes':sum(x['size'] for x in entries),'verified':False,'originals_removed':False,'checkpoint':'964e6417d14fbe71cd59668cba78113914615a3a','stage':stage,'supporting_receipts':receipt_names,'supporting_receipt_sha256':{name:digest(root/name) for name in receipt_names},'restore_command':f'tar -xzf {archive} -C {root}'}
free_before=shutil.disk_usage(root).free
assert free_before>=max(64*1024*1024,record['original_file_bytes']+32*1024*1024),'hold archive if insufficient conservative temporary headroom'
durable_json(index_path,record)
started=time.monotonic()
with tarfile.open(temporary,'w:gz',compresslevel=1,dereference=True) as out:
    for path in files:out.add(path,arcname=str(path.relative_to(root)),recursive=False)
with temporary.open('rb') as synced:os.fsync(synced.fileno())
expected={entry['path']:entry for entry in entries};seen=set()
with tarfile.open(temporary,'r:gz') as source:
    for member in source:
        assert member.name in expected and member.name not in seen;seen.add(member.name)
        entry=expected[member.name]
        assert member.isfile() and member.mode==entry['mode'] and member.size==entry['size'],member.name
        assert stream_digest(source.extractfile(member))==(entry['sha256'],entry['size']),member.name
assert seen==set(expected)
with gzip.open(temporary,'rb') as source:tar_sha,tar_size=stream_digest(source)
record.update(verified=True,archive_sha256=digest(temporary),archive_bytes=temporary.stat().st_size,decompressed_tar_sha256=tar_sha,decompressed_tar_bytes=tar_size)
os.replace(temporary,archive);fsync_directory(archive.parent)
durable_json(index_path,record)
durable_json(tracked_index,record)
# Every recovery index and archive has reached durable storage before any unlink.
for entry in entries:
    path=Path(entry['source']);info=path.lstat()
    assert stat.S_ISREG(info.st_mode) and info.st_nlink==1 and info.st_size==entry['size'] and stat.S_IMODE(info.st_mode)==entry['mode'] and digest(path)==entry['sha256'],str(path)
for entry in entries:
    path=Path(entry['source']);path.unlink();fsync_directory(path.parent)
record.update(originals_removed=True,elapsed_seconds=time.monotonic()-started,free_bytes_before=free_before,free_bytes_after=shutil.disk_usage(root).free)
record['global_free_change_bytes']=record['free_bytes_after']-free_before
record['own_reclaimed_blocks_before_indexes_bytes']=sum(e['allocated_bytes'] for e in entries)-archive.stat().st_blocks*512
durable_json(index_path,record);durable_json(tracked_index,record)
summary={key:value for key,value in record.items() if key!='entries'}
summary['entry_count']=len(entries);summary['recovery_index']=str(tracked_index.relative_to(root));summary['recovery_index_sha256']=digest(tracked_index)
durable_json(receipt,summary)
print(json.dumps(summary),flush=True)
