#!/usr/bin/env python3
"""Archive an explicit verified inventory of closed Split/ByteView functional ELF/IR files without touching protected inputs."""
from __future__ import annotations
import argparse
import gzip
import json
from pathlib import Path
import os
import shutil
import stat

from archive_byte_ascii_functional import ROOT, digest, save, sync_directory, verify


def main() -> None:
    """Require full report linkage, original hashes/modes and a durable verified recovery record before every unlink."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--inventory',type=Path,required=True)
    parser.add_argument('--index',type=Path,required=True)
    parser.add_argument('--first',type=int,default=0)
    parser.add_argument('--limit',type=int)
    args = parser.parse_args()
    inventory_path = args.inventory.resolve()
    inventory = json.loads(inventory_path.read_text())
    assert inventory['logical_bytes']<=300*1024*1024
    assert inventory.get('cumulative_logical_bytes',inventory['logical_bytes'])<=300*1024*1024
    pair_path = Path('/workspace/dasl-stdlib-refined-mask/build/v4-legacy-2f077-pair/pair.json')
    assert digest(pair_path)=='cb76fd09ca5ef0ad6cca30ee081948dd641807ef441729e71aac5e251fe9730d'
    protected = {item['path'] for item in json.loads(pair_path.read_text())['verified_files']}
    selected = inventory['members'][args.first:]
    if args.limit is not None:
        selected = selected[:args.limit]
    index = args.index.resolve()
    assert index.is_relative_to(ROOT/'build')
    index.parent.mkdir(parents=True,exist_ok=True)
    archives = index.parent/'files'
    archives.mkdir(exist_ok=True)
    sync_directory(index.parent)
    sync_directory(index.parent.parent)
    state = json.loads(index.read_text()) if index.exists() else dict(schema=1,scope='explicit successful older Split/ByteView corpus ELF/IR only',files=[])
    before = shutil.disk_usage(ROOT).free
    count = 0
    original_bytes = 0
    for item in selected:
        original = ROOT/item['path']
        assert original.is_relative_to(ROOT/'build') and str(original) not in protected
        assert original.name in ['consumer','consumer.ll'] and original.parent.name in [
            'rows','owners','byte-split-v4-original-native-o2-rows',
            'byte-split-v4-initial-owners','byte-split-v4-original-native-o2-owners']
        assert not any(word in item['path'] for word in ['bench','frozen','provider','compiler','runtime','progress','regression','failed','interface'])
        report = ROOT/item['receipt']
        assert digest(report)==item['receipt_sha256']
        result = json.loads(report.read_text())
        if 'results' in result:
            group = next(value for value in result['results'] if value['group']==item['group'])
            profiles = group[item.get('profile_field','profiles')]
            expected = group['llvm_sha256'] if original.suffix=='.ll' else group['executable_sha256']
        else:
            assert result['status']=='passed'
            profiles = result['profiles']
            expected = result['llvm_sha256'] if original.suffix=='.ll' else result['exe_sha256']
        assert expected==item['sha256'] and len(profiles)==item['profiles'] and all(value['zero_leaks'] for value in profiles)
        previous = next((value for value in state['files'] if value['path']==item['path']),None)
        if previous:
            assert previous['sha256']==item['sha256']
            verify(ROOT/previous['archive'],previous)
            entry = previous
        else:
            metadata = original.lstat()
            assert stat.S_ISREG(metadata.st_mode) and metadata.st_size==item['size']
            assert stat.S_IMODE(metadata.st_mode)==item['mode'] and digest(original)==item['sha256']
            archive = archives/(item['sha256']+'.gz')
            if archive.exists():
                equivalent = next(value for value in state['files'] if value['sha256']==item['sha256'])
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
            entry = dict(path=item['path'],size=item['size'],mode=item['mode'],sha256=item['sha256'],
                         archive=str(archive.relative_to(ROOT)),compressed_sha256=digest(archive),decompressed_sha256=item['sha256'],
                         compressed_bytes=archive.stat().st_size,receipt=item['receipt'],receipt_sha256=item['receipt_sha256'],
                         receipt_group=item['group'],memory_profiles=item['profiles'],original_unlinked=False)
            verify(archive,entry)
            state['files'].append(entry)
        save(index,state)
        if original.exists():
            metadata = original.lstat()
            assert stat.S_ISREG(metadata.st_mode) and metadata.st_size==entry['size']
            assert stat.S_IMODE(metadata.st_mode)==entry['mode'] and digest(original)==entry['sha256']
            original.unlink()
            sync_directory(original.parent)
            count += 1
            original_bytes += entry['size']
        entry['original_unlinked']=True
        save(index,state)
    print(json.dumps(dict(files_archived=count,original_logical_bytes=original_bytes,before_free_bytes=before,
                          after_free_bytes=shutil.disk_usage(ROOT).free,index=str(index))))


if __name__=='__main__':
    main()
