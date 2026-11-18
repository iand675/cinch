#!/usr/bin/env python3
"""Generate review-only IO+Mut buffered source copies and an exact public signature impact manifest."""
from pathlib import Path
import hashlib,json,re,subprocess

ROOT=Path(__file__).resolve().parents[2]
HERE=Path(__file__).resolve().parent
BASELINE='8d0557bddc624a4c700c6849c676e8d5cdef9582'
MODULES=['Effect.Buffered.Core','Effect.Buffered.Reader','Effect.Buffered.Writer','Effect.Scanner']
CHANGED={
'Effect.Buffered.Core':{'buffer-source-with-transfer','buffer-sink-with-transfer','buffer-source-read','source-proxy-read','source-proxy-transfer','sink-proxy-transfer','run-source-transfer','run-sink-transfer','source-event','buffer-source-transfer','buffer-sink-transfer','attached-sink-transfer'},
'Effect.Buffered.Reader':{'fill-raw','fill','read-empty','buffered-read','peek-fill','buffered-peek','discard-from','buffered-discard','read-byte-from','buffered-read-byte','buffered-read-into','rune-fill','buffered-read-rune','slice-from','buffered-read-slice','collect-bytes','buffered-read-bytes','buffered-read-line','buffered-read-string','reader-transfer','write-to-loop','write-to-dispatch','buffered-write-to'},
'Effect.Buffered.Writer':{'writer-transfer','read-source','read-from-step','read-from-loop','buffered-read-from'},
'Effect.Scanner':{'read-more','refill','advance-token','evaluate','scan-loop','scanner-next'},
}


def sha(text):
    """Identify exact UTF-8 source bytes before and after the isolated transformation."""
    return hashlib.sha256(text.encode()).hexdigest()


def signatures(text):
    """Observe only declared exported value signatures, keeping private storage constructors separate."""
    exported=set()
    for names in re.findall(r'^  export (.*)$',text,re.M):exported.update(name.strip() for name in names.split(','))
    return {name:sig for name,sig in re.findall(r'^  ([\w-]+) :: (.*)$',text,re.M) if name in exported}


def outer_both(line):
    """Widen the enclosing operation row while preserving independent IO-only write/text/flush callbacks."""
    before,sep,after=line.rpartition('->{IO r}')
    if not sep:raise ValueError('no enclosing IO operation in '+line)
    return before+'->{IO r, Mut r}'+after


def transform(module,text):
    """Propagate fixed both-effect reads without pretending that a general row-indexed source is supported."""
    lines=[]
    for line in text.splitlines():
        found=re.match(r'^  ([\w-]+) :: ',line)
        if found and found[1] in CHANGED[module]:line=outer_both(line)
        if module in ['Effect.Buffered.Core','Effect.Buffered.Writer']:
            line=line.replace('(Nat ->{IO r} ReadEvent)','(Nat ->{IO r, Mut r} ReadEvent)')
        if module=='Effect.Buffered.Core':
            line=line.replace('ReadEvent) ->{IO r} (TransferEvent','ReadEvent) ->{IO r, Mut r} (TransferEvent')
        if module=='Effect.Buffered.Core' and re.match(r'^    (SourceTransferStep|SourceProxyStep) ::',line):
            line=line.replace('Nat ->{IO r} (ReadEvent','Nat ->{IO r, Mut r} (ReadEvent')
            line=line.replace('WriteEvent) ->{IO r} (','WriteEvent) ->{IO r, Mut r} (')
        if module=='Effect.Buffered.Core' and re.match(r'^    (SinkTransferStep|SinkProxyStep) ::',line):
            line=line.replace('ReadEvent) ->{IO r} (','ReadEvent) ->{IO r, Mut r} (')
        lines.append(line)
    text='\n'.join(lines)+'\n'
    if module=='Effect.Buffered.Core':
        text=text.replace('  export BufferSource, BufferSink, buffer-source, buffer-sink','  export BufferSource, BufferSink, buffer-source, buffer-sink\n  export buffer-source-with-mutation')
        anchor='    SourceStep :: (Nat ->{IO r} (ReadEvent, BufferSource r)) -> BufferSource r'
        assert anchor in text
        text=text.replace(anchor,anchor+'\n    -- | A live read may perform I/O and mutation in the same region without erasing either effect.\n    SourceMutStep :: (Nat ->{IO r, Mut r} (ReadEvent, BufferSource r)) -> BufferSource r')
        anchor='  buffer-source step = SourceStep step'
        assert anchor in text
        text=text.replace(anchor,anchor+'''\n
  -- | Construct a live source whose native reads require both I/O and mutation in the same region.
  buffer-source-with-mutation :: forall r. (Nat ->{IO r, Mut r} (ReadEvent, BufferSource r)) -> BufferSource r
  buffer-source-with-mutation step = SourceMutStep step

  -- | Execute an IO-only callback inside a conservatively both-effect read without narrowing its type.
  widen-source-step :: forall r. (Nat ->{IO r} (ReadEvent, BufferSource r)) -> Nat ->{IO r, Mut r} (ReadEvent, BufferSource r)
  widen-source-step step wanted = step wanted
''')
        anchor='    SourceStep step -> SourceTransferStep step transfer'
        assert anchor in text
        text=text.replace(anchor,'    SourceStep step -> SourceTransferStep (widen-source-step step) transfer\n    SourceMutStep step -> SourceTransferStep step transfer')
        anchor='    SourceStep step -> False';assert anchor in text
        text=text.replace(anchor,anchor+'\n    SourceMutStep step -> False')
        anchor='    SourceStep step -> step wanted';assert anchor in text
        text=text.replace(anchor,anchor+'\n    SourceMutStep step -> step wanted')
        anchor='    SourceStep step -> (None, source)';assert anchor in text
        text=text.replace(anchor,anchor+'\n    SourceMutStep step -> (None, source)')
    if module in ['Effect.Buffered.Reader','Effect.Buffered.Writer']:
        text=text.replace('buffer-source (\\wanted ->','buffer-source-with-mutation (\\wanted ->')
    return text


def main():
    """Refuse a moved production baseline and emit a manifest suitable for review before any API promotion."""
    output=HERE/'sources';output.mkdir(exist_ok=True)
    report={'status':'isolated review experiment; frozen production API untouched; no source/runtime/effect gates claimed yet','baseline':BASELINE,'modules':{},'public_changes':[],'new_public_values':[],'preserved':['legacy Effect.Stream unchanged','ordinary sink writes/text/flush remain IO-only','pure getters/configuration remain pure','retained callback contracts unchanged','no runtime/ABI/checker/store changes'],'conservative_effect_policy':'A general source can become mutable on any continuation. General reads and transfers therefore require IO+Mut even when a particular call is currently satisfied by an IO-only source or buffered bytes.','higher_order_impact':'Sink.ReaderFrom must accept a read callback with IO+Mut. Merely widening its outer operation would still erase the callback effect.','promotion':'Review exact signature cost and both frontend checks before considering promotion; original22-cycle hold remains.'}
    for module in MODULES:
        relative='compiler/prelude/'+module.replace('.','/')+'.cinch'
        before=subprocess.check_output(['git','show',BASELINE+':'+relative],cwd=ROOT).decode()
        if (ROOT/relative).read_text()!=before:raise ValueError('frozen API source moved: '+relative)
        after=transform(module,before);(output/(module+'.cinch')).write_text(after)
        old=signatures(before);new=signatures(after)
        changes=[{'module':module,'name':name,'old':old[name],'new':new[name],'already_both_effects':False} for name in sorted(old) if old[name]!=new[name]]
        report['public_changes'].extend(changes)
        report['new_public_values'].extend({'module':module,'name':name,'signature':new[name]} for name in sorted(new.keys()-old.keys()))
        report['modules'][module]={'old_sha256':sha(before),'new_sha256':sha(after),'frozen_source':relative,'experiment_source':'sources/'+module+'.cinch'}
    report['changed_existing_public_signatures']=len(report['public_changes'])
    assert report['changed_existing_public_signatures']==18,report['public_changes']
    assert len(report['new_public_values'])==1
    (HERE/'signature-impact.json').write_text(json.dumps(report,indent=2)+'\n')
    print('Prepared exact18-signature experiment plus one source factory; frozen API unchanged')


if __name__=='__main__':main()
