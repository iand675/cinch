"""Run the current parent palette target with bounded disk use and retained receipts."""
from pathlib import Path
import hashlib,json,os,shutil,signal,subprocess,time
ROOT=Path('/workspace/dasl-stdlib-io')
WORK=ROOT/'compiler/repros/palette-parent-v1'
WORK.mkdir(exist_ok=False)

def digest(path):
    """Hash the exact artifact used or produced by this gate."""
    return hashlib.sha256(path.read_bytes()).hexdigest()

def save(record):
    """Persist a receipt atomically while preserving the unmodified command output."""
    target=WORK/'receipt.json'; temporary=WORK/'receipt.next.json'
    with temporary.open('w') as out:
        json.dump(record,out,indent=2);out.write('\n');out.flush();os.fsync(out.fileno())
    os.replace(temporary,target)

def size(paths):
    """Charge new output to physical inodes once, including compiler intermediates."""
    result={}
    for base in paths:
        for path in base.rglob('*'):
            try:
                if path.is_file():
                    stat=path.stat();result[(stat.st_dev,stat.st_ino)]=stat.st_size
            except FileNotFoundError:
                pass
    return result

roots=[ROOT/'seed/target',ROOT/'seed/runtime/target',WORK]
original=size(roots)
free=shutil.disk_usage(ROOT).free
assert free>=350*1024*1024
runtime=ROOT/'seed/runtime/target/release/libcinchruntime.a'
frozen=ROOT/'build/stdlib-private-usage-producer-frozen/cinch'
record={'scope':'Current Rust parent palette documentation and SDK-free runtime integration; retained complete project, no Native/foreign execution/performance claim.', 'command':['/tmp/cinch-stdlib-root-cargo-gate.sh','test','--release','--test','lib_image_color_palette','--','--nocapture','--test-threads=1'],'cwd':str(ROOT/'compiler'),'initial_free_bytes':free,'reserve_bytes':250*1024*1024,'soft_pause_below_bytes':270*1024*1024,'fresh_output_cap_bytes':160*1024*1024,'runtime_sha256_before':digest(runtime),'frozen_producer_sha256_before':digest(frozen),'inputs':{},'status':'running','pauses':[]}
for rel in ['seed/tests/lib_image_color_palette.rs','seed/prelude/Effect/Random.cinch','experiments/image-color-palette-v1/sources/Image.Color.Palette.cinch','experiments/image-color-v1/sources/Image.Color.cinch','tooling/check_color_palette_ci.py','tooling/check_color_palette.py','tooling/check_compare.py']:
    record['inputs'][rel]=digest(ROOT/rel)
log=(WORK/'cargo.log').open('w');started=time.monotonic();paused=0
process=subprocess.Popen(record['command'],cwd=record['cwd'],stdout=log,stderr=subprocess.STDOUT,start_new_session=True)
record['process_group']=process.pid;save(record);minimum=free;maximum=0;reason=None
while process.poll() is None:
    current=shutil.disk_usage(ROOT).free;minimum=min(minimum,current)
    fresh=sum(length for inode,length in size(roots).items() if inode not in original);maximum=max(maximum,fresh)
    if fresh>record['fresh_output_cap_bytes'] or current<record['soft_pause_below_bytes']:
        os.killpg(process.pid,signal.SIGSTOP);pause=time.monotonic();record['status']='resource-paused';record['pauses'].append({'free_bytes':current,'fresh_output_bytes':fresh});save(record)
        print(json.dumps({'resource_paused':True,'group':process.pid,'free_bytes':current,'fresh_output_bytes':fresh,'resume_marker':str(WORK/'RESUME')}),flush=True)
        while not (WORK/'RESUME').exists():time.sleep(.5)
        (WORK/'RESUME').unlink();paused+=time.monotonic()-pause;record['status']='running';save(record);os.killpg(process.pid,signal.SIGCONT)
    if time.monotonic()-started-paused>900:
        reason='900 second active deadline';os.killpg(process.pid,signal.SIGTERM);break
    time.sleep(.3)
try:code=process.wait(timeout=10)
except subprocess.TimeoutExpired:
    os.killpg(process.pid,signal.SIGKILL);code=process.wait()
log.close()
record.update(status='passed' if code==0 and reason is None else 'failed',exit_code=code,guard_reason=reason,wall_seconds=time.monotonic()-started,paused_seconds=paused,minimum_free_bytes=minimum,maximum_new_output_bytes=maximum,end_free_bytes=shutil.disk_usage(ROOT).free,runtime_sha256_after=digest(runtime),frozen_producer_sha256_after=digest(frozen))
record['inputs_unchanged']=all(digest(ROOT/path)==value for path,value in record['inputs'].items())
assert record['frozen_producer_sha256_before']==record['frozen_producer_sha256_after']
save(record);print(json.dumps(record));print((WORK/'cargo.log').read_text()[-6000:]);raise SystemExit(code or (1 if reason else 0))
