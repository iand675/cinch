#!/usr/bin/env python3
"""Archive only closed successful PCG functional outputs; retain every input and failed epoch plain."""
from pathlib import Path
import argparse,hashlib,json,os,stat,zstandard


def digest(path):
    """Hash exact file bytes before any destructive storage replacement."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def sync_directory(path):
    """Order durable payload and index creation before removal of a plain output."""
    fd=os.open(path,os.O_RDONLY|os.O_DIRECTORY)
    try:os.fsync(fd)
    finally:os.close(fd)


def main():
    """Verify both runtime profiles and all source bindings before creating recovery payloads."""
    parser=argparse.ArgumentParser(description=__doc__);parser.add_argument('work',type=Path);args=parser.parse_args()
    work=args.work.resolve();receipt=work/'receipt.json';qualified=json.loads(receipt.read_text())
    assert work.name.startswith('pcg-functional-') and qualified['status']=='passed'
    assert len(qualified['runs'])==2
    for profile in qualified['runs']:
        assert profile['returncode']==0 and profile['stdout']=='' and 'live=0' in profile['stderr'] and 'leaked=0' in profile['stderr']
    for name,value in qualified['sources'].items():assert digest(work/name)==value
    archive=work/'closed-output-archive';archive.mkdir(exist_ok=False)
    index={'scope':__doc__,'execution_receipt_sha256':digest(receipt),'files':[],'ordering':'Full decoded identity, compressed SHA, original metadata and durable payload/index verified before unlink. Restore decompresses exact bytes, then reapplies mode/uid/gid/nsmtime.','sources_and_interfaces_remain_plain':True}
    charged=0
    for name,key in [('consumer.ll','llvm_sha256'),('consumer','executable_sha256')]:
        original=work/name;assert digest(original)==qualified[key];info=original.stat();target=archive/(name+'.zst')
        with original.open('rb') as source,target.open('xb') as output:
            zstandard.ZstdCompressor(level=3,write_checksum=True).copy_stream(source,output);output.flush();os.fsync(output.fileno())
        charged+=target.stat().st_size;assert charged<=10*1024*1024
        decoded=hashlib.sha256();length=0
        with target.open('rb') as source,zstandard.ZstdDecompressor().stream_reader(source) as stream:
            while block:=stream.read(1024*1024):decoded.update(block);length+=len(block)
        assert decoded.hexdigest()==qualified[key] and length==info.st_size
        index['files'].append({'path':str(original),'sha256':qualified[key],'bytes':info.st_size,'mode':stat.S_IMODE(info.st_mode),'uid':info.st_uid,'gid':info.st_gid,'mtime_ns':info.st_mtime_ns,'original_device':info.st_dev,'original_inode':info.st_ino,'archive':str(target),'compressed_sha256':digest(target),'decoded_sha256_verified':True})
    sync_directory(archive)
    with (archive/'index.json').open('x') as output:json.dump(index,output,indent=2);output.write('\n');output.flush();os.fsync(output.fileno())
    sync_directory(archive)
    for entry in index['files']:
        original=Path(entry['path']);info=original.stat();assert digest(original)==entry['sha256'] and info.st_size==entry['bytes'] and info.st_mtime_ns==entry['mtime_ns'] and info.st_ino==entry['original_inode']
    for entry in index['files']:Path(entry['path']).unlink()
    sync_directory(work)
    print(json.dumps({'status':'verified-archived-success','index':str(archive/'index.json'),'index_sha256':digest(archive/'index.json'),'payload_bytes':charged,'net_logical_bytes':sum(entry['bytes'] for entry in index['files'])-charged}))


if __name__=='__main__':main()
