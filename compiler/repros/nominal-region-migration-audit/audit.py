"""Find actual checked nominal Type headers applied to Region-bound scheme variables; never restamp them."""
from pathlib import Path
import hashlib,json,re
root=Path('/workspace/dasl-stdlib-io');interfaces=root/'build/nominal-region-library-imports/build'
def parse_groups(text):
    """Retain balanced rendered application groups without interpreting source declarations."""
    stack=[[]]
    for token in re.findall(r'[()]|[^\s()]+',text):
        if token=='(':group=[];stack[-1].append(group);stack.append(group)
        elif token==')':
            assert len(stack)>1;stack.pop()
        else:stack[-1].append(token)
    assert len(stack)==1
    return stack[0]
def applications(node):
    """Extract fully parenthesized left-associated applications from a checked scheme."""
    def flatten(value):
        if isinstance(value,str):return value,[]
        if len(value)==2:
            result=flatten(value[0])
            if result:return result[0],result[1]+[value[1]]
        return None
    if isinstance(node,list):
        result=flatten(node)
        if result:yield result
        for child in node:yield from applications(child)
records={};inputs=[]
for path in sorted(interfaces.glob('*.cinchif')):
    raw=path.read_bytes();doc=json.loads(raw);inputs.append({'module':doc['module'],'sha256':hashlib.sha256(raw).hexdigest(),'source_hash':doc['source_hash']})
    for value in doc['exports']['values']:
        resolved=value.get('resolved')
        if not resolved:continue
        scheme=resolved['scheme'];regions=set(re.findall(r'\(([^\s()]+)\s+::\s+Region\)',scheme));nominals={n['identity']:n for n in resolved['nominals']}
        if not regions:continue
        for name,args in applications(parse_groups(scheme)):
            nominal=nominals.get(name)
            if not nominal:continue
            parameters=nominal['parameters']
            if len(args)!=len(parameters):continue
            for position,(parameter,arg) in enumerate(zip(parameters,args)):
                if parameter['kind']=='Type' and isinstance(arg,str) and arg in regions:
                    key=(name,position);row=records.setdefault(key,{'identity':name,'parameter':parameter['name'],'position':position,'recorded_kind':'Type','uses':[]})
                    witness={'module':doc['module'],'value':value['name'],'region_argument':arg,'scheme':scheme}
                    if witness not in row['uses']:row['uses'].append(witness)
result={'status':'Advisory defining-source migration audit; no source or interface edited','interface_epoch':'Genuine aggregate190-module Role check before the subsequent Slots/Array codegen-only repairs','scope':'A complete checked scheme applies a nominal recorded Type parameter to a variable explicitly quantified as Region. Historical headers remain authoritative; propose explicit source declarations and regenerate/requalify each closure.','module_interfaces':len(inputs),'nominal_parameter_candidates':len(records),'inputs':inputs,'candidates':[records[k] for k in sorted(records)]}
out=root/'compiler/evidence/stdlib/stdlib-nominal-region-migration-audit.json';out.write_text(json.dumps(result,indent=2)+'\n');print(json.dumps({'interfaces':len(inputs),'candidates':len(records),'defining_modules':sorted({x[0].rsplit('.',1)[0] for x in records})}))
