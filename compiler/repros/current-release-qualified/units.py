"""Run the release unit gate with a measured disk budget and preserved failures."""
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import time

ROOT = Path('/workspace/dasl-stdlib-io')
WORK = ROOT / 'build/stdlib-current-release-units'
TARGET = ROOT / 'seed/target/release'
MIB = 1024 * 1024


def allocated(paths):
    """Count physical storage once per inode, including compiler scratch files."""
    seen = set()
    total = 0
    for path in paths:
        files = path.rglob('*') if path.is_dir() else [path]
        for item in files:
            try:
                stat = item.stat()
                key = (stat.st_dev, stat.st_ino)
                if item.is_file() and key not in seen:
                    seen.add(key)
                    total += stat.st_blocks * 512
            except FileNotFoundError:
                pass
    return total


assert TARGET.exists(), 'The qualified release build cache must exist.'
assert shutil.disk_usage(ROOT).free >= 800 * MIB
WORK.mkdir(parents=True, exist_ok=False)
scratch = WORK / 'scratch'
scratch.mkdir()
log = WORK / 'cargo.log'
command = ['/tmp/cinch-stdlib-root-cargo-gate.sh', 'test', '--locked', '--offline', '--release', '--lib']
report = {'command': command, 'initial_free_bytes': shutil.disk_usage(ROOT).free,
          'start_threshold_bytes': 800 * MIB, 'reserve_bytes': 350 * MIB,
          'new_allocated_cap_bytes': 800 * MIB,
          'scope': 'Monitored release unit gate; allocation cap includes the existing release cache. Compiler checks and profile flags are unchanged.',
          'samples': []}
started = time.monotonic()
with log.open('wb') as output:
    process = subprocess.Popen(command, cwd=ROOT / 'compiler', start_new_session=True,
                               stdout=output, stderr=subprocess.STDOUT,
                               env={**os.environ, 'TMPDIR': str(scratch)})
    report['pid'] = process.pid
    while process.poll() is None:
        sample = {'elapsed_seconds': time.monotonic() - started,
                  'free_bytes': shutil.disk_usage(ROOT).free,
                  'allocated_bytes': allocated([TARGET, WORK])}
        report['samples'].append(sample)
        reason = None
        if sample['free_bytes'] < report['reserve_bytes']:
            reason = 'global reserve'
        elif sample['allocated_bytes'] > report['new_allocated_cap_bytes']:
            reason = 'stage output cap'
        if reason:
            report['stopped_reason'] = reason
            os.killpg(process.pid, signal.SIGTERM)
            try:
                process.wait(timeout=10)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.wait()
        (WORK / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
        if reason:
            break
        time.sleep(2)
    report['returncode'] = process.wait()
    report['elapsed_seconds'] = time.monotonic() - started
    output.flush()
    os.fsync(output.fileno())
report['final_free_bytes'] = shutil.disk_usage(ROOT).free
report['final_allocated_bytes'] = allocated([TARGET, WORK])
(WORK / 'report.json').write_text(json.dumps(report, indent=2) + '\n')
print(json.dumps({key: value for key, value in report.items() if key != 'samples'}))
raise SystemExit(0 if report['returncode'] == 0 else 1)
