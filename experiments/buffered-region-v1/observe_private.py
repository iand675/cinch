#!/usr/bin/env python3
"""Inspect the private broadcast carrier in a separately identified observation copy of an admitted source closure."""
from pathlib import Path
import argparse
import json
import os
import shutil
import subprocess
import time
import check


def main():
    """Expose only a pure identity for metadata observation, without changing the actual candidate's API."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--source-work', type=Path, required=True)
    parser.add_argument('--work', type=Path, required=True)
    parser.add_argument('--deadline', type=int, default=60)
    args = parser.parse_args()
    os.sched_setaffinity(0, {2})
    source_work = args.source_work.resolve()
    source_report_path = source_work / 'report.json'
    source_report = json.loads(source_report_path.read_text())
    assert source_report['source_gate_passed'] and source_report['compiler_sha256'] == check.COMPILER
    assert source_report['runtime_sha256'] == check.RUNTIME
    proposal_path = check.HERE / 'proposal.json'
    assert source_report['proposal_sha256'] == check.digest(proposal_path)
    assert check.digest(source_work / 'Main.cinch') == source_report['main_sha256']
    for name, row in source_report['source_selection'].items():
        assert check.digest(source_work / (name + '.cinch')) == row['sha256'], name
    compiler = check.SNAPSHOT / 'cinch'
    assert check.digest(compiler) == check.COMPILER
    assert check.digest(check.SNAPSHOT / 'libcinchruntime.a') == check.RUNTIME
    assert shutil.disk_usage(check.ROOT).free >= 350 * 1024 * 1024, '350MiB fresh-start guard'
    work = args.work.resolve()
    assert not work.exists(), 'refuse to overwrite a historical observation epoch'
    work.mkdir(parents=True)
    for path in source_work.glob('*.cinch'):
        shutil.copyfile(path, work / path.name)
    target = work / 'Effect.Stream.cinch'
    original_sha = check.digest(target)
    observed = target.read_text() + '''
  export observe-writer-chain

  -- | Observe a private carrier's defining Region contract without performing an operation.
  observe-writer-chain :: forall (r :: Region). WriterChain r -> WriterChain r
  observe-writer-chain value = value
'''
    target.write_text(observed)
    command = [str(compiler), 'check', str(work / 'Main.cinch')]
    check.guard(work)
    before = time.monotonic()
    timeout = False
    try:
        result = subprocess.run(command, cwd=work, capture_output=True, text=True,
                                preexec_fn=check.limits, timeout=args.deadline,
                                env={**os.environ, 'CINCH_STACK_SIZE': '64M', 'RUST_MIN_STACK': '67108864'})
        code, stdout, stderr = result.returncode, result.stdout, result.stderr
    except subprocess.TimeoutExpired as error:
        timeout = True
        code = None
        stdout = error.stdout or b''
        stderr = error.stderr or b''
        stdout = stdout.decode() if isinstance(stdout, bytes) else stdout
        stderr = stderr.decode() if isinstance(stderr, bytes) else stderr
    report = {'scope': __doc__, 'production_adopted': False, 'runtime_claim': False,
              'compiler_sha256': check.COMPILER, 'runtime_sha256': check.RUNTIME,
              'source_gate_sha256': check.digest(source_report_path),
              'proposal_sha256': check.digest(proposal_path),
              'candidate_stream_sha256': original_sha, 'observation_stream_sha256': check.digest(target),
              'observer': 'Effect.Stream.observe-writer-chain', 'command': command,
              'returncode': code, 'timeout_without_verdict': timeout,
              'stdout': stdout, 'stderr': stderr, 'seconds': time.monotonic() - before}
    report_path = work / 'report.json'
    report_path.write_text(json.dumps(report, indent=2) + '\n')
    if code == 0 and not timeout:
        interface_path = work / 'build/Effect.Stream.cinchif'
        interface = json.loads(interface_path.read_text())
        observer = next(row for row in interface['exports']['values'] if row['name'] == 'observe-writer-chain')
        nominal = next(row for row in observer['resolved']['nominals'] if row['identity'] == 'Effect.Stream.WriterChain')
        assert nominal['parameters'] == [{'implicit': False, 'kind': 'Region', 'name': 'r'}]
        assert nominal['constructors'] == ['Effect.Stream.WriterEnd', 'Effect.Stream.WriterLink']
        assert 'WriterChain' not in {row['name'] for row in interface['exports']['types']}
        report['private_defining_contract'] = nominal
        report['private_carrier_visibility'] = 'remains private; observation copy exports only the identity function'
        report['observation_interface_sha256'] = check.digest(interface_path)
        report['public_contracts'] = check.public_contracts(work, json.loads(proposal_path.read_text()))
        report['genuine_interfaces'] = {p.name: check.digest(p) for p in (work / 'build').glob('*.cinchif')}
        report['artifact_bytes'] = sum(p.stat().st_size for p in work.rglob('*') if p.is_file())
        report_path.write_text(json.dumps(report, indent=2) + '\n')
    check.guard(work)
    print(stdout + stderr or ('Deadline without verdict' if timeout else 'Private WriterChain Region defining contract accepted'), flush=True)
    raise SystemExit(124 if timeout else code)


if __name__ == '__main__':
    main()
