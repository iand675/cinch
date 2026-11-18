#!/usr/bin/env python3
"""Emit/link/run only the exact admitted855/c4 addressability snapshot."""
from pathlib import Path
import hashlib
import json
import os
import re
import resource
import subprocess
import time
root = Path('/workspace/dasl-stdlib-json')
directory = root / 'build/json-method-owner-probe-v1'
probe = json.loads((directory / 'report.json').read_text())
compiler = Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen/cinch')
runtime = compiler.parent / 'libcinchruntime.a'
clang = Path('/workspace/.tools/llvm/usr/lib/llvm-20/bin/clang')
output = directory / 'consumer'
ir = directory / 'consumer.ll'
receipt = root / 'bootstrap/stdlib-json-method-owner-rust-v1-results.json'
def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()
def within_budget():
    count = sum(path.stat().st_size for path in directory.rglob('*') if path.is_file())
    assert count <= 20 * 1024 * 1024, f'artifact budget exceeded: {count}'
    return count
assert probe['exit_code'] == 0
assert not ir.exists() and not output.exists() and not receipt.exists(), 'refuse accidental repeated qualification'
assert digest(compiler) == probe['compiler_sha256'] == '855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e'
assert digest(runtime) == probe['provenance']['runtime_sha256'] == 'c4b1097b4b8e26541400228bdbbdcba1376101014823aeb2f5af619fbede57ac'
for name, expected in probe['sources'].items():
    assert digest(directory / (name + '.cinch')) == expected
assert digest(directory / 'Main.cinch') == probe['fixture_sha256']
free=__import__('shutil').disk_usage(root).free
assert free>=250*1024*1024, 'Root guard:require250MiB before runtime unit'
os.sched_setaffinity(0, {2})
_, hard = resource.getrlimit(resource.RLIMIT_STACK)
resource.setrlimit(resource.RLIMIT_STACK, (65536 * 1024, hard))
env = dict(os.environ)
env['LD_LIBRARY_PATH'] = '/workspace/.tools/llvm/usr/lib/x86_64-linux-gnu' + (':' + env['LD_LIBRARY_PATH'] if env.get('LD_LIBRARY_PATH') else '')
emit = [str(compiler), 'emit-ir', str(directory / 'Main.cinch'), '-o', str(ir), '--rc-debug']
started = time.monotonic()
emitted = subprocess.run(emit, env=env, cwd=directory, text=True, capture_output=True, timeout=30)
(directory / 'emit.log').write_text(emitted.stdout + emitted.stderr)
assert emitted.returncode == 0, emitted.stdout + emitted.stderr
within_budget()
link = [str(clang), '-O2', str(ir), str(runtime), '-ldl', '-lpthread', '-lm', '-o', str(output)]
linked = subprocess.run(link, env=env, cwd=directory, text=True, capture_output=True, timeout=30)
(directory / 'link.log').write_text(linked.stdout + linked.stderr)
assert linked.returncode == 0, linked.stderr
report = {'description':'NEW exact855/c4 current-source directional5 +metadata ownership3 controls; no full1464 claim', 'frontend':'rust', 'classification_epoch':'855/c4 for NEW source only; historical7e/2f unchanged', 'compiler_sha256':digest(compiler), 'provenance':probe['provenance'], 'observed_runtime_archive_sha256':digest(runtime), 'sources':probe['sources'], 'fixture_sha256':probe['fixture_sha256'], 'declaration_receipt_sha256':digest(directory/'report.json'), 'runner_sha256':digest(Path(__file__)), 'raw_compile_command':emit, 'raw_link_command':link, 'llvm_sha256':digest(ir), 'clang_sha256':digest(clang), 'opt_level':2, 'cpu_affinity':[2], 'compiler_host_stack_kib':65536, 'consumer_stack_limit':'8M', 'build_seconds':time.monotonic()-started, 'artifact_bytes':within_budget(), 'runs':[]}
receipt.write_text(json.dumps(report,indent=2)+'\n')
for poisoned in [False, True]:
    run_env = {**env, 'CINCH_HEAP_LIMIT':'256M' if poisoned else '32M', 'CINCH_STACK_SIZE':'8M', 'CINCH_LEAK_CHECK':'1', 'CINCH_RC_DEBUG':'1' if poisoned else '0', 'CINCH_ALLOC_STATS':'1'}
    run = subprocess.run([str(output)],env=run_env,cwd=directory,text=True,capture_output=True,timeout=15)
    (directory/('poisoned.log' if poisoned else 'ordinary.log')).write_text(run.stdout+run.stderr)
    assert run.returncode == 0, f'exit {run.returncode}: {run.stdout}\n{run.stderr}'
    assert run.stdout == '' and 'leaked=0' in run.stderr, run.stdout+run.stderr
    line=next(line for line in run.stderr.splitlines() if line.startswith('cinch-alloc:'))
    stats={key:int(value) for key,value in re.findall(r'([a-z-]+)=(\d+)',line)}
    assert stats['live'] == 0, stats
    report['runs'].append({'poisoned_free_validation':poisoned,'heap_limit':run_env['CINCH_HEAP_LIMIT'],'stdout':run.stdout,'stats':stats,'leaked':0})
    receipt.write_text(json.dumps(report,indent=2)+'\n')
assert digest(runtime)==probe['provenance']['runtime_sha256']
report['status']='passed'
receipt.write_text(json.dumps(report,indent=2)+'\n')
print('Exact855/c4 method owner controls:directional5 +metadata3 pass both profiles, zero live/leaks.',flush=True)
