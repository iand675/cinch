#!/usr/bin/env python3
"""Check one frozen next-source addressability closure; no IR or host link."""
from pathlib import Path
import hashlib
import importlib.util
import json
import os
import resource
import subprocess
root = Path('/workspace/dasl-stdlib-json')
spec = importlib.util.spec_from_file_location('json_validator', root / 'tooling/validate_json_codecs.py')
validator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(validator)
modules = dict(validator.MODULES)
for name in ['Text.Base64.Encoding', 'Text.Json.Codec']:
    modules[name] = root / 'seed/prelude' / (name.replace('.', '/') + '.cinch')
directory = root / 'build/json-error-pointer-probe-v5'
assert not directory.exists(), 'refuse repeated probe'
directory.mkdir()
compiler = Path('/workspace/dasl-stdlib-io/build/boxed-nat-memory-safe-frozen/cinch')
hashfile = lambda p: hashlib.sha256(p.read_bytes()).hexdigest()
assert hashfile(compiler) == '855df83617f2ea84e7f0e3cf90b9755685bc23226594d81cd43ccd6168071b4e'
for name, source in modules.items():
    (directory / (name + '.cinch')).write_bytes(source.read_bytes())
fixture = root / 'seed/tests/fixtures/json-codec-error-pointer.cinch'
(directory / 'Main.cinch').write_bytes(fixture.read_bytes())
os.sched_setaffinity(0, {2})
_, hard = resource.getrlimit(resource.RLIMIT_STACK)
resource.setrlimit(resource.RLIMIT_STACK, (65536 * 1024, hard))
command = [str(compiler), 'check', str(directory / 'Main.cinch')]
run = subprocess.run(command, capture_output=True, text=True, timeout=15, cwd=directory)
(directory / 'check.log').write_text(run.stdout + run.stderr)
report = {'description':'NEW quoted-field288 pointer method fallback/error/prefix/callback observations declaration/type/proof only; no IR/runtime claim', 'compiler_sha256':hashfile(compiler), 'provenance':json.loads((compiler.parent/'provenance.json').read_text()), 'sources':{name:hashfile(path) for name,path in modules.items()}, 'fixture_sha256':hashfile(fixture), 'command':command, 'exit_code':run.returncode, 'diagnostic':run.stdout + run.stderr}
(directory/'report.json').write_text(json.dumps(report,indent=2)+'\n')
assert sum(p.stat().st_size for p in directory.iterdir() if p.is_file()) < 5*1024*1024, 'source-only size budget'
print(run.stdout + run.stderr)
assert run.returncode == 0, 'stop first diagnostic'
