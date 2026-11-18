#!/usr/bin/env python3
"""Check the isolated both-effect multipart review without emission or execution."""
from __future__ import annotations
import argparse
import gzip
import hashlib
import io
import json
import os
from pathlib import Path
import re
import resource
import shutil
import subprocess
import tarfile
import time
from bench_stdlib import source_closure
from check_compare import digest


#: Review files are separate from the selected production modules and frozen evidence.
ROOT = Path(__file__).resolve().parents[1]


def archive_cache(work: Path, selector: str, proof: dict) -> None:
    """Archive closed generated caches, reusing exact verified members before durable unlink."""
    cache = work / 'build'
    files = sorted(path for path in cache.rglob('*') if path.is_file())
    if not files:
        return
    known = {}
    for index in work.glob('*.index.json'):
        data = json.loads(index.read_text())
        if 'archive' not in data or 'members' not in data:
            continue
        archive = Path(data['archive'])
        if digest(archive) != data['compressed_sha256']:
            raise RuntimeError('changed checked-cache archive')
        decoded = gzip.decompress(archive.read_bytes())
        if hashlib.sha256(decoded).hexdigest() != data['decoded_tar_sha256']:
            raise RuntimeError('changed decoded checked-cache archive')
        with tarfile.open(fileobj=io.BytesIO(decoded), mode='r') as stored:
            for row in data['members']:
                member = stored.getmember(row['relative'])
                content = stored.extractfile(member).read()
                if hashlib.sha256(content).hexdigest() != row['sha256'] or len(content) != row['bytes'] or member.mode != row['mode']:
                    raise RuntimeError('changed exact checked-cache member')
                known[(row['sha256'], row['bytes'], row['mode'])] = {'archive': str(archive), 'index': str(index), 'member': row['relative']}
    rows = [{'path': str(path), 'relative': str(path.relative_to(cache)), 'sha256': digest(path),
             'bytes': path.stat().st_size, 'mode': path.stat().st_mode & 0o777} for path in files]
    fresh = [row for row in rows if (row['sha256'], row['bytes'], row['mode']) not in known]
    label = selector.replace(':', '-')
    if fresh:
        payload = io.BytesIO()
        with tarfile.open(fileobj=payload, mode='w') as stored:
            for row in fresh:
                stored.add(row['path'], arcname=row['relative'], recursive=False)
        original = payload.getvalue()
        archive = work / (label + '-cache.tar.gz')
        with archive.open('wb') as raw:
            with gzip.GzipFile(filename='', fileobj=raw, mode='wb', compresslevel=1, mtime=0) as output:
                output.write(original)
            raw.flush(); os.fsync(raw.fileno())
        decoded = gzip.decompress(archive.read_bytes())
        if decoded != original:
            raise RuntimeError('checked-cache roundtrip differs')
        with tarfile.open(fileobj=io.BytesIO(decoded), mode='r') as stored:
            for row in fresh:
                member = stored.getmember(row['relative'])
                content = stored.extractfile(member).read()
                if hashlib.sha256(content).hexdigest() != row['sha256'] or len(content) != row['bytes'] or member.mode != row['mode']:
                    raise RuntimeError('new checked-cache member roundtrip differs')
        index = work / (label + '-cache.index.json')
        with index.open('w') as output:
            json.dump({'archive': str(archive), 'compressed_sha256': digest(archive),
                       'decoded_tar_sha256': hashlib.sha256(decoded).hexdigest(), 'decoded_tar_bytes': len(decoded), 'members': fresh}, output, indent=2)
            output.write('\n'); output.flush(); os.fsync(output.fileno())
        for row in fresh:
            known[(row['sha256'], row['bytes'], row['mode'])] = {'archive': str(archive), 'index': str(index), 'member': row['relative']}
    for row in rows:
        row['recovery'] = known[(row['sha256'], row['bytes'], row['mode'])]
    recovery = work / (label + '-cache-recovery.json')
    with recovery.open('w') as output:
        json.dump({'scope': 'Closed generated check-cache only; actual input sources remain plain.', 'proof': proof, 'files': rows}, output, indent=2)
        output.write('\n'); output.flush(); os.fsync(output.fileno())
    directory = os.open(work, os.O_RDONLY | os.O_DIRECTORY)
    try:
        os.fsync(directory)
        for path in files:
            path.unlink()
        for folder in sorted((path for path in cache.rglob('*') if path.is_dir()), key=lambda path: len(path.parts), reverse=True):
            folder.rmdir()
        cache.rmdir()
        os.fsync(directory)
    finally:
        os.close(directory)


def main() -> None:
    """Reuse one bounded check cache while freezing every actual input and refusal."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--rust', type=Path, required=True)
    parser.add_argument('--native', type=Path, required=True)
    parser.add_argument('--cpu', type=int, required=True)
    parser.add_argument('--only', nargs='+', help='Explicit frontend:client selectors')
    parser.add_argument('--archive-check-cache', action='store_true', help='Verify durable cold-cache recovery before unlink; actual sources stay plain')
    args = parser.parse_args()
    os.sched_setaffinity(0, {args.cpu})
    resource.setrlimit(resource.RLIMIT_STACK, (67108864, resource.getrlimit(resource.RLIMIT_STACK)[1]))
    experiment = ROOT / 'experiments/multipart-both-effects-v1'
    work = args.work.resolve(); work.mkdir(parents=True, exist_ok=True)
    clients = sorted((experiment / 'clients').glob('*.cinch'))
    selected = [client.read_text() for client in clients]
    overrides = {name: experiment / 'sources' / (name + '.cinch') for name in ('Text.Multipart', 'Effect.Multipart')}
    closure = source_closure('\n'.join(selected), overrides)
    ordered, visited = [], set()

    def visit(name: str) -> None:
        """Freeze only the actual dependency order accepted by the source-distributed frontend."""
        if name in visited:
            return
        visited.add(name)
        for dependency in re.findall(r'^\s*import\s+([A-Z][\w.]*)', closure[name].read_text(), re.M):
            if dependency in closure:
                visit(dependency)
        ordered.append(name)

    for name, path in closure.items():
        visit(name)
        shutil.copyfile(path, work / (name + '.cinch'))
    prefix = '\n'.join((work / (name + '.cinch')).read_text() for name in ordered) + '\n'
    (work / 'native-prefix.source').write_text(prefix)
    report = {'scope': __doc__, 'runtime_claim': False, 'production_adopted': False,
              'compiler_host_stack_bytes': 67108864, 'cpu_affinity': [args.cpu],
              'signature_impact_sha256': digest(experiment / 'signature-impact.json'),
              'sources': {name: digest(work / (name + '.cinch')) for name in ordered},
              'native_prefix_sha256': digest(work / 'native-prefix.source'), 'runs': []}
    snapshot = args.native.resolve()
    report['native_pair'] = {name: digest(snapshot / name) for name in ('cinch', 'base-interfaces.json', 'objects.json', 'libcinchruntime.a')}
    for frontend, compiler in [('rust', args.rust.resolve()), ('native', snapshot / 'cinch')]:
        for client in clients:
            selector = frontend + ':' + client.stem
            if args.only and selector not in args.only:
                continue
            entry = work / 'Main.cinch'; shutil.copyfile(client, entry)
            if frontend == 'rust':
                command = [str(compiler), 'check', str(entry)]
            else:
                bundle = work / 'bundle.source'; bundle.write_text(prefix + client.read_text())
                command = [str(compiler), '--check', '--interfaces', str(snapshot / 'base-interfaces.json'),
                           str(bundle), str(work / 'check.output')]
            diagnostic = work / 'check.output.diagnostics'
            if diagnostic.exists():
                diagnostic.unlink()
            started = time.monotonic()
            result = subprocess.run(command, cwd=work, text=True, capture_output=True, timeout=60,
                                    env={**os.environ, 'RUST_MIN_STACK': '67108864', 'CINCH_STACK_SIZE': '64M'})
            details = diagnostic.read_text() if diagnostic.exists() else ''
            row = {'selector': selector, 'compiler_sha256': digest(compiler), 'client_path': str(client),
                   'client_sha256': digest(client), 'prefix_sha256': report['native_prefix_sha256'] if frontend == 'native' else None,
                   'command': command, 'returncode': result.returncode, 'stdout': result.stdout,
                   'stderr': result.stderr, 'diagnostics': details, 'seconds': time.monotonic() - started}
            expected_positive = client.stem == 'positive'
            row['expected'] = 'accepted full contract' if expected_positive else 'refused IO-only factory without effect erasure'
            storage_row_refusal = frontend == 'native' and client.stem == 'negative-storage' and re.fullmatch(
                r'bootstrap.check:\d+:\d+: type mismatch: expected \{\(IO r\)\}, found \{\(IO r\), \(Mut r\)\}\n?', details) is not None
            row['matched'] = result.returncode == 0 if expected_positive else result.returncode != 0 and (
                any(word in result.stderr + details for word in ('effect row', 'effect inclusion')) or storage_row_refusal)
            report['runs'].append(row)
            if args.archive_check_cache:
                archive_cache(work, selector, row)
            report['artifact_bytes'] = sum(path.stat().st_size for path in work.rglob('*') if path.is_file())
            (work / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
            print(selector, 'accepted' if result.returncode == 0 else 'refused', flush=True)
            if not row['matched']:
                raise RuntimeError('unexpected source-check boundary: ' + result.stderr + details)
            if report['artifact_bytes'] > 5 * 1024 * 1024:
                raise RuntimeError('source-check artifact cap reached; preserve boundary before further checks')


if __name__ == '__main__':
    main()
