"""Generate real Cinch library documentation from Rust-checked interfaces.

python3 scripts/generate-library-reference.py --compiler /path/to/cinch
python3 scripts/generate-library-reference.py --check

The generator uses a temporary project and private package store. Non-embedded
modules are copied by declared name beside the entry, matching the compiler's
consumer tests. No repository compiler sources or caches are modified.
"""
from pathlib import Path
import argparse
import hashlib
import json
import os
import re
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
PRELUDE = ROOT / 'compiler/prelude'
OUTPUT = ROOT / 'library'
MANIFEST = ROOT / 'website/src/data/library-sources.json'
DESCRIPTIONS = {
'Prelude':'The implicit, curated base surface available in every program. This facade re-exports selected defining modules; it is not the whole library.',
'Data.Core':'Shared declarations for List, Option, and Result. Their public homes add operations and instances; this module breaks declaration-order cycles.',
'Data.List':'Finite strict lists: construction, transformation, filtering, splitting, searching, folds, and collection instances.',
'Data.Option':'Optional values with None and Some, plus combinators for propagating or consuming absence.',
'Data.Result':'Success and named failure values, error-aware transformations, partitioning, and sequencing instances.',
'Data.Enum':'Finite enumeration and the class behind inclusive range literals, including an explicit next element for a step.',
'Data.Nat':'Mathematical nonnegative integers, exact arithmetic, checked preconditions, and admitted machine conversions.',
'Data.Int':'Mathematical signed integers, arithmetic, division/remainder, formatting support, and explicit conversions.',
'Data.Machine':'Fixed-width and target-sized scalar operations. Names make checked, wrapping, or saturating overflow policy explicit.',
'Data.Bool':'Boolean operations and their comparison instances.',
'Data.Ordering':'The LT, EQ, and GT comparison result vocabulary and its instances.',
'Data.Char':'Unicode scalar values and checked character/code conversions, with range information for safe use.',
'Data.String':'Immutable UTF-8 text, encoding/decoding, character access, slicing, comparison, and concatenation. Byte conversions copy to preserve immutability.',
'Data.Bytes':'Mutable byte buffers with length, construction, copy, and bounds/range-aware access operations.',
'Data.Map':'Ordered maps with comparison constraints, lookup, insertion, removal, traversal, and collection-literal support.',
'Data.Set':'Ordered sets and their membership, combination, difference, and conversion operations.',
'Data.Hash':'Hashing vocabulary and admitted hash implementations for keys.',
'Data.HashMap':'Hash maps with explicit key constraints, lookup/update, and map-style combination operations.',
'Data.Sym':'Interned symbols with identity equality, deterministic spelling order, and String conversions.',
'Data.NonEmpty':'Lists refined by nonemptiness. The total constructor returns Option; transferred operations must preserve the predicate.',
'Data.NonZero':'Total nonzero-number constructors. The NonZero type is also available through the arithmetic vocabulary for safe divisors.',
'Data.Sorted':'Lists refined by sortedness, with construction and combination that preserve ordering.',
'Data.Ascii':'ASCII-refined values and checked construction at text boundaries.',
'Data.Identity':'The identity container and its generic abstraction instances.',
'Data.Const':'A constant container that retains its first parameter while ignoring the mapped value parameter.',
'Data.Compose':'Composition of higher-kinded containers with the admitted generic instances.',
'Data.FromList':'The opt-in collection-literal vocabulary used to interpret [elements] at the expected container type.',
'Data.FromMap':'The opt-in map-literal vocabulary used to interpret {key => value} at the expected container type.',
'Data.Path':'Platform path values and composition/conversion operations used by filesystem and process APIs.',
'Data.Json':'The JSON value representation shared by parsers, renderers, codecs, and structured observability.',
'Data.Producer':'Partial producer vocabulary, distinct from codata whose next observation is guaranteed productive.',
'Data.Target':'Target/platform and worker vocabulary used by the compiler-supported runtime and target examples.',
'Data.Rational':'Rational numbers with explicit denominator/error policy and arithmetic operations.',
'Data.BigDecimal':'Decimal arithmetic with explicit precision/scale and rounding/error behaviour in the exported API.',
'Data.Complex':'Complex values and numeric operations using the module\'s current floating representation.',
'Data.Matrix':'Matrix construction, shape checks, indexing, transformation, and arithmetic operations.',
'Data.Date':'Civil date values, validation, parsing/formatting, and calendar calculations.',
'Data.Time':'Time values, durations, and conversions with explicit units and admitted ranges.',
'Data.IpAddr':'Validated IP addresses, parsing/rendering, and address-family vocabulary.',
'Data.StringIo':'In-memory string-oriented I/O utilities with explicit state and error results.',
'Function.Combinators':'Small reusable function operations such as identity, composition, argument manipulation, and application.',
'Relation.Eq':'Equality/inequality class vocabulary. Instances live with their types.',
'Relation.Ord':'Ordering vocabulary and comparison helpers. State all required dictionaries explicitly; superclass entailment is not implicit.',
'Algebra.Arithmetic':'Arithmetic operation signatures and their fixities, including NonZero divisors and InRange refinements.',
'Algebra.Bits':'Bit operations, shifts, and the corresponding operator/fixity vocabulary.',
'Algebra.Semigroup':'Associative combination vocabulary and its law. Reported finite checks are distinct from universal proofs.',
'Algebra.Monoid':'Identity plus combination vocabulary, default folding, and unit laws with explicit checking status.',
'Algebra.Group':'Group-style inverse/subtraction vocabulary over admitted carrier instances.',
'Algebra.Semiring':'Zero/one and additive/multiplicative structure, with its law vocabulary.',
'Category.Functor':'Higher-kinded mapping vocabulary, distinct from the lexical do-sequencing provider mechanism.',
'Category.Applicative':'Container application and value lifting for independent structure-preserving composition.',
'Category.Monad':'Container-dependent composition through the admitted higher-kinded class vocabulary.',
'Category.Alternative':'Empty and choice operations for admitted higher-kinded containers.',
'Category.Foldable':'Generic folds, conversion to lists, size/emptiness, and related operations over container instances.',
'Category.Traversable':'Generic effect/container-aware traversal and sequencing while retaining the input shape.',
'Effect.Sequencing':'The direct, Result, and Option do providers. One lexical provider/container is selected per definition.',
'Effect.Concurrent':'Shipped task/worker vocabulary supplementing the compiler-installed structured scope and join operations.',
'Effect.Io':'A worked configuration-reading API over real filesystem bytes; parse-config treats a nonempty payload as configuration.',
'Effect.Clock':'Runtime monotonic and wall-clock readings under an I/O capability. Clocks are not ambient compile-time inputs.',
'Effect.Fs':'Filesystem reads/writes, directory and metadata operations, with Path values and named Result errors.',
'Effect.Process':'One-shot and live subprocesses, command arguments/environment additions, pipes, exit codes, and this process\'s arguments.',
'Effect.Net':'Socket operations with explicit handles, address values, errors, and runtime effects.',
'Effect.Http':'A bounded plaintext HTTP client with response framing and named errors. HTTPS is explicitly refused; this module has no TLS implementation.',
'Effect.Resolv':'Hostname/address resolution through the runtime, with named failure results.',
'Effect.Signal':'Operating-system signal vocabulary and runtime operations with explicit error/effect handling.',
'Effect.Pty':'Pseudo-terminal-backed child processes, terminal I/O, and lifecycle operations.',
'Effect.Random':'Runtime entropy/random utilities with explicit effectful access.',
'Effect.FileUtils':'Convenience filesystem operations built on the lower-level filesystem API.',
'Effect.Find':'Filesystem walking/search utilities with the module\'s traversal and error policy.',
'Effect.Log':'Structured log operations and records with runtime capability/effect accounting.',
'Effect.Trace':'Scoped tracing/span vocabulary and context propagation tied to the runtime identity.',
'Effect.Test':'A subprocess-based test runner and named test-result/report vocabulary.',
'Effect.Benchmark':'Benchmark case execution and timing/reporting helpers over runtime clocks.',
'Effect.Coverage':'Coverage collection/reporting utilities with explicit runtime and source metadata.',
'Text.Show':'Rendering values as String. The concrete formatting helpers are re-exported by Prelude; import this module for Show itself.',
'Text.Codec':'Encoding/decoding signatures and codec helpers with explicit failure results.',
'Text.Lex':'A source-distributed lexer toolkit: tokens, spans, and lexical errors for compiler/parser clients.',
'Text.Scan':'Scanning primitives over input text and the shared position/error vocabulary.',
'Text.Parse':'A source-distributed parser toolkit over the lexer/scanner vocabulary.',
'Text.Json':'JSON parsing and rendering over Data.Json, with explicit errors and builder-based output.',
'Text.Builder':'An efficient text-construction vocabulary for assembling many fragments before rendering.',
'Text.Regex':'Regular-expression parsing/matching and replacement under the module\'s admitted regex language.',
'Text.Args':'Command-line argument processing and its named value/error vocabulary.',
'Text.OptionParser':'Declarative option specifications, short/long forms, conversion, positionals, usage, and named parse errors.',
'Text.Csv':'CSV parsing/rendering with explicit field, record, and malformed-input handling.',
'Text.Yaml':'The module\'s supported YAML subset, structured values, and named parse errors; not an assertion of every YAML feature.',
'Text.Toml':'The module\'s supported TOML parsing/rendering surface and explicit unsupported/malformed cases.',
'Text.Uri':'URI parsing, rendering, components, and validation/error vocabulary.',
'Text.Base64':'Base64 encoding/decoding over bytes with explicit invalid-input results.',
'Text.Digest':'Digest and HMAC operations over bytes. Availability of a legacy digest is not a recommendation for a security protocol.',
'Text.Shellwords':'Shell-style word splitting/joining; this is data parsing, not execution of a shell command.',
'Text.Zlib':'Compression/decompression operations and named malformed/unsupported-stream results.',
'Text.Otlp':'OTLP/JSON record construction/rendering for structured log/trace data and trace-context helpers.',
}

def sources():
    return {'.'.join(p.relative_to(PRELUDE).with_suffix('').parts): p for p in sorted(PRELUDE.rglob('*.cinch'))}

def digest(path): return hashlib.sha256(path.read_bytes()).hexdigest()
def anchor(kind, name): return f'{kind}-{name.encode().hex()}'
def heading(kind, name): return f'<a id="{anchor(kind, name)}"></a>\n\n## `{name}`\n'
def code(text): return f'```cinch\n{text}\n```\n'
def public_class(item): return bool(item.get('exports'))
def markdown_text(text): return text.replace('|', '\\|').replace('\n', ' ')

def source_comments(text):
    lines=text.splitlines(); found={}
    for i,line in enumerate(lines):
        match=re.match(r'\s*(?:foreign import ccall "[^"]+"\s+)?(\([^\s]+\)|[\w\'-]+)\s*::',line)
        decl=re.match(r'\s*(?:data|record|codata|signature|type)\s+([\w\'-]+)',line)
        name=(match or decl)
        if not name:continue
        name=name[1].strip('()'); comments=[];j=i-1
        while j>=0 and (lines[j].lstrip().startswith('--') or not lines[j].strip()):
            value=lines[j].strip()
            comments.append(re.sub(r'^-- ?', '',value) if value else '')
            j-=1
        description='\n'.join(reversed(comments)).strip()
        # Separator-only comment bars are visual source organization, not prose.
        description='\n'.join(x for x in description.splitlines() if not re.fullmatch(r'[-= ]+',x))
        found.setdefault(name, {'line':i+1,'doc':description.strip()})
    return found

def collect_interfaces(compiler):
    temp=tempfile.TemporaryDirectory(prefix='cinch-library-')
    work=Path(temp.name);manifest=json.loads((PRELUDE/'cinch-base.cinchpkg').read_text())
    embedded={m['name'] for m in manifest['modules']}
    all_sources=sources()
    for name,path in all_sources.items():
        if name not in embedded:(work/(name+'.cinch')).write_bytes(path.read_bytes())
    env={**os.environ,'CINCH_STORE':str(work/'store')}
    entry=work/'Main.cinch'
    def check(name):
        entry.write_text('module Main where\n'+(f'  import {name}\n' if name else '')+'  main :: Int\n  main = 0\n')
        result=subprocess.run([compiler,'check',str(entry)],cwd=work,env=env,capture_output=True,text=True,timeout=90)
        if result.returncode: raise RuntimeError(f'Rust compiler rejected {name or "Prelude"}:\n{result.stderr}')
        print('Checked '+(name or 'embedded base'),flush=True)
    check(None)
    for name in all_sources:
        if name not in embedded:check(name)
    return temp,work/'build'

def generate(ifaces):
    all_sources=sources()
    embedded={m['name'] for m in json.loads((PRELUDE/'cinch-base.cinchpkg').read_text())['modules']}
    interfaces={name:json.loads((ifaces/(name+'.cinchif')).read_text()) for name in all_sources}
    comments={name:source_comments(path.read_text()) for name,path in all_sources.items()}
    downloads=ROOT/'website/public/reference-sources';downloads.mkdir(parents=True,exist_ok=True)
    def info(identity,name):
        module=identity.rsplit('.',1)[0] if '.' in identity else ''
        return comments.get(module,{}).get(name,{})
    def details(identity,name):
        item=info(identity,name)
        prose=item.get('doc','')
        module=identity.rsplit('.',1)[0] if '.' in identity else ''
        source_link=f'/reference-sources/{module}.cinch' if module in all_sources else ''
        parts=[]
        if prose:parts.append(prose)
        if source_link:parts.append(f'[Source]({source_link})'+(f' · line {item["line"]}' if item.get('line') else ''))
        return '\n\n'.join(parts)+'\n'
    total_values=0;total_methods=0; entries=[]
    for name,path in all_sources.items():
        iface=interfaces[name];export=iface['exports'];text=path.read_text()
        (downloads/(name+'.cinch')).write_text(text)
        embedded_here=name in embedded
        imports=sorted(set(re.findall(r'^  import ([\w.]+)',text,re.M)))
        closure=set()
        def deps(module):
            if module in closure or module in embedded or module not in all_sources:return
            closure.add(module)
            for child in re.findall(r'^  import ([\w.]+)',all_sources[module].read_text(),re.M):deps(child)
        deps(name)
        lines=[f'# {name}','',DESCRIPTIONS[name],'', '<span class="support-tag support-tag--current">Embedded</span> Available from the compiler\'s embedded base family.' if embedded_here else '<span class="support-tag support-tag--source">Ship source</span> Copy this module and its non-embedded dependency closure beside your entry file. An import does not download it.', '',f'Import with `import {name}`. '+('Selected exports may already be in Prelude.' if name!='Prelude' else 'This module is implicitly available.'),'',f'[Download exact module source](/reference-sources/{name}.cinch) · [How library modules are loaded](00-index.md)','']
        if imports:lines+=['# Dependencies','',', '.join(f'[{i}]({i}.md)' if i in all_sources else f'`{i}` (compiler-generated vocabulary)' for i in imports)+'.','']
        if not embedded_here:
            lines+=['Non-embedded source closure: '+', '.join(f'[{m}]({m}.md)' for m in sorted(closure))+'.','']
        header=re.split(r'^module ', text, maxsplit=1, flags=re.M)[0]
        header='\n'.join(re.sub(r'^-- ?', '',l) for l in header.splitlines()).strip()
        if header:lines+=['<details><summary>Behaviour and boundary notes from the source</summary>','',header,'','</details>','']
        lines+=['# Types and constructors','']
        if not export['types']:lines+=['This module exports no type declarations.','']
        for ty in export['types']:
            if ty.get('kind')=='signature':continue
            lines+=[heading('type',ty['name'])]
            params=' '.join(f'({p["name"]} :: {p["kind"]})' for p in ty.get('parameters',[]))
            refinement=ty.get('refinement')
            if refinement:
                lines+=[code(f'type {ty["name"]} {params} = refine ({refinement["binder"]} :: {refinement["carrier"]}) with {refinement["predicate"]}')]
            else:
                lines+=[code(f'{ty["kind"]} {ty["name"]} {params}'.rstrip())]
            lines+=[details(ty.get('identity',name+'.'+ty['name']),ty['name'])]
            for ctor in ty.get('ctors',[]):
                lines+=[code(f'{ctor["name"]} :: {ctor.get("scheme") or " -> ".join(ctor.get("fields",[]) + [ty["name"]])}')]
                doc=info(ctor.get('symbol',name+'.'+ctor['name']),ctor['name']).get('doc')
                if doc:lines+=[doc,'']
            if ty.get('fields'):
                lines+=['Fields:','',code('\n'.join(f'{f["name"]} :: {f.get("ty",f.get("scheme",""))}' if isinstance(f,dict) else str(f) for f in ty['fields']))]
        classes=[s for s in export['signatures'] if public_class(s) or any(m.get('exports') for m in s['methods'])]
        if classes:lines+=['# Signatures and methods','']
        for sig in classes:
            lines+=[heading('class',sig['name']),code(f'signature {sig["name"]} '+ ' '.join(sig.get('params',[]))),details(sig.get('identity',name+'.'+sig['name']),sig['name'])]
            for method in sig['methods']:
                if not method.get('exports'):continue
                total_methods+=1
                lines+=[heading('method',sig['name']+'.'+method['name']),code(f'{method["name"]} :: {method["scheme"]}'),details(sig.get('identity',name+'.'+sig['name']),method['name'])]
        if export['values']:lines+=['# Values','']
        for value in export['values']:
            total_values+=1
            symbol=value.get('symbol',name+'.'+value['name'])
            if name=='Prelude':continue
            lines+=[heading('value',value['name']),code(f'{value["name"]} :: {value["scheme"]}'),details(symbol,value['name'])]
        if name=='Prelude':
            lines+=['This facade re-exports the following checked values. Follow a defining module for declaration documentation. Compiler-installed operations such as `ioWrite` have a [separate reference](../reference/20-compiler-vocabulary.md).','','| Value | Defining module |','| --- | --- |']
            for value in export['values']:
                home=value.get('symbol','').rsplit('.',1)[0]
                link=f'[{home}]({home}.md#{anchor("value",value["name"])})' if home in all_sources else f'`{home}`'
                lines.append(f'| `{markdown_text(value["name"])}` | {link} |')
            lines+=['']
        if export['fixities']:
            lines+=['# Operator fixities','',code('\n'.join(f'{ {"left":"infixl","right":"infixr","none":"infix"}.get(f.get("assoc"), "infix")} {f.get("prec",0)} {f.get("op","")}' for f in export['fixities']))]
        if export['instances']:
            lines+=['# Instances','','These are checked exported instance entries. Law/sample-check status is separate from the existence of an instance; see [laws and optimization](../reference/12-rewrites-and-optimization.md).','','| Instance | Target | Resolution |','| --- | --- | --- |']
            for inst in export['instances']:
                lines.append(f'| `{markdown_text(inst["id"])}` | `{markdown_text(inst["target"])}` | '+('Implicit' if inst['implicit'] else 'Explicit')+' |')
            lines+=['']
        lines+=['# Reference provenance','',f'Generated from the Rust compiler\'s checked exports and `compiler/prelude/{path.relative_to(PRELUDE)}`. Source SHA-256: `{digest(path)}`. This records the source snapshot, not a released compiler version. Regenerate after changing library sources; do not edit generated declarations by hand.','']
        (OUTPUT/(name+'.md')).write_text('\n'.join(lines))
        entries.append({'module':name,'path':str(path.relative_to(ROOT)),'sha256':digest(path),'embedded':embedded_here,'values':len(export['values']),'public_methods':sum(bool(m.get('exports')) for s in export['signatures'] for m in s['methods'])})
    manifest={'format':1,'base_manifest_sha256':digest(PRELUDE/'cinch-base.cinchpkg'),'modules':entries}
    MANIFEST.write_text(json.dumps(manifest,indent=2)+'\n')
    index=['# Built-in and shipped libraries','','The actual library source distributed with the **Rust compiler**, documented from checked compiler interfaces. This directory is separate from the website\'s [sample package catalogue](/packages/).','','# Choose a module','','Use site search to find a declaration by name or signature. The groups below distinguish embedded modules from source-distributed modules. Public types, constructors, values, methods, fixities, and instance entries come from `.cinchif` exports; source comments explain behaviour and named limits. Private class closure in an interface is not presented as a public method API.','','# Load embedded modules','','The compiler embeds the modules marked **Embedded**. Import their defining home when a name is absent from the curated Prelude. Compiler-installed special forms and operations such as `newIO`, `ioWrite`, `spawn`, and `decide` are documented in [compiler vocabulary](../reference/20-compiler-vocabulary.md).','','# Use source-distributed modules','','Modules marked **Ship source** are implemented and check with the Rust compiler, but are not embedded. Put their source and non-embedded dependency closure beside your entry file. For example, from the repository root:','','```sh','mkdir -p /tmp/cinch-json-example','cp compiler/prelude/Text/Lex.cinch /tmp/cinch-json-example/Text.Lex.cinch','cp compiler/prelude/Text/Scan.cinch /tmp/cinch-json-example/Text.Scan.cinch','cp compiler/prelude/Text/Builder.cinch /tmp/cinch-json-example/Text.Builder.cinch','cp compiler/prelude/Text/Parse.cinch /tmp/cinch-json-example/Text.Parse.cinch','cp compiler/prelude/Text/Json.cinch /tmp/cinch-json-example/Text.Json.cinch','```','','Save a `Main.cinch` there and use `import Text.Json`. Each module page lists its actual imports and the complete non-embedded source closure. The module declaration determines identity; the dotted filenames above make the sibling-directory arrangement unambiguous. These copy commands do not install a remote package.','','<span class="support-tag support-tag--planned">Coming soon</span> a complete remote package-install/publish workflow and real binary release bundles. The current local package manifest/store is described in [modules](../reference/07-modules-and-packages.md) and [artifacts](../reference/18-artifacts-and-incremental.md).','','# Module directory','']
    groups=['Prelude','Algebra','Category','Relation','Function','Data','Text','Effect']
    for group in groups:
        subset=[e for e in entries if e['module'].split('.')[0]==group]
        index += [f'## {group}','','| Module | Distribution | Purpose |','| --- | --- | --- |']
        for e in subset:
            name=e['module'];index.append(f'| [{name}]({name}.md) | '+('Embedded' if e['embedded'] else 'Ship source')+f' | {DESCRIPTIONS[name]} |')
        index+=['']
    index+=['# Scope and verification','',f'This generation covers **{len(entries)} source modules**: **{sum(e["embedded"] for e in entries)} embedded** and **{sum(not e["embedded"] for e in entries)} source-distributed**. Each source-distributed module was imported in an isolated consumer project and checked by the Rust compiler before extraction. Checking an import establishes accepted declarations; it is not an end-to-end runtime test of every function. Runnable tutorial examples are verified separately.','','Signatures preserve the compiler\'s rendered schemes, including constraints and effect rows. When a source comment and a checked scheme differ, treat the scheme and Rust behaviour as current and investigate the source note. Availability of a module does not imply every feature of another language\'s similarly named library (for example, plaintext HTTP is not TLS, and a YAML parser may support a subset).','','Regenerate with `python3 website/scripts/generate-library-reference.py --compiler /path/to/cinch`. Check source drift with `python3 website/scripts/generate-library-reference.py --check`.','']
    (OUTPUT/'00-index.md').write_text('\n'.join(index))
    print(f'Generated {len(entries)} module pages, {total_values} value exports, {total_methods} public method entries')

def main():
    args=argparse.ArgumentParser(description=__doc__)
    args.add_argument('--compiler')
    args.add_argument('--interfaces',type=Path,help='Use a previously checked interface directory from the same source snapshot')
    args.add_argument('--check',action='store_true')
    options=args.parse_args()
    if options.check:
        recorded=json.loads(MANIFEST.read_text())
        actual=sources()
        assert set(actual)=={m['module'] for m in recorded['modules']},'Library source set changed; regenerate reference'
        assert recorded['base_manifest_sha256']==digest(PRELUDE/'cinch-base.cinchpkg'),'Base manifest changed; regenerate reference'
        for entry in recorded['modules']:
            assert entry['sha256']==digest(actual[entry['module']]),f'{entry["module"]}: source changed; regenerate reference'
            assert (OUTPUT/(entry['module']+'.md')).exists(),f'Missing module page: {entry["module"]}'
            assert (ROOT/'website/public/reference-sources'/(entry['module']+'.cinch')).read_bytes()==actual[entry['module']].read_bytes(),f'Stale source download: {entry["module"]}'
        print(f'Library reference matches all {len(actual)} module sources and the base manifest')
        return
    temp=None
    if options.interfaces:
        ifaces=options.interfaces
    else:
        if not options.compiler:args.error('supply --compiler to the Rust cinch executable')
        temp,ifaces=collect_interfaces(str(Path(options.compiler).resolve()))
    try:generate(ifaces)
    finally:
        if temp:temp.cleanup()

if __name__=='__main__':main()
