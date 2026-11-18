#!/usr/bin/env python3
"""Prepare source-only suffix lookup groups and independently validate unchanged oracle ordering without invoking Cinch."""
from __future__ import annotations
import hashlib
import json
from pathlib import Path
from validate_literals import consumer

HERE=Path(__file__).resolve().parent


def digest(path: Path) -> str:
    """Bind static grouping evidence to exact oracle and generated source bytes."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    """Validate every literal observation with naive Python suffix sorting, then render bounded consumers."""
    vectors=HERE/'vectors.json';rows=json.loads(vectors.read_text())['cases'];queries=0;differences=[]
    for number,row in enumerate(rows):
        data=bytes.fromhex(row['data']);order=sorted(range(len(data)),key=lambda at:data[at:])
        assert (row['suffix_order'] or [])==order,(number,'independent suffix order')
        for query in row['queries']:
            needle=bytes.fromhex(query['needle']);matches=[at for at in order if needle and data[at:].startswith(needle)]
            expected=matches if query['limit']<0 else matches[:query['limit']]
            assert (query['offsets'] or [])==expected,(number,query)
            queries+=1
        for regex in row['regex']:
            if regex['indexed']!=regex['leftmost']:
                differences.append(dict(corpus_input=number,pattern=regex['pattern'],limit=regex['limit'],indexed=regex['indexed'],leftmost=regex['leftmost']))
    output=HERE/'consumers';output.mkdir(exist_ok=True);groups=[]
    for start in range(0,len(rows),8):
        path=output/f'literals-{start:04}.cinch';path.write_text(consumer(rows[start:start+8]))
        groups.append(dict(start=start,count=min(8,len(rows)-start),source=path.name,sha256=digest(path),bytes=path.stat().st_size))
    report=dict(scope='Static consumer preparation only; no Cinch frontend/runtime/timing',status='prepared',vectors_sha256=digest(vectors),inputs=len(rows),literal_observations_independently_crosschecked=queries,groups=groups,bounded_regexp_choice_differences=len(differences),bounded_regexp_difference_examples=differences[:5],interpretation='SDK bounded suffix candidate selection may choose later matches; the explicit leftmost callback adaptation is not falsely equated with these bounded choices.')
    (HERE/'prepared-groups.json').write_text(json.dumps(report,indent=2)+'\n')
    print(json.dumps(dict(inputs=len(rows),literal_observations=queries,groups=len(groups),generated_source_bytes=sum(x['bytes'] for x in groups),bounded_regexp_choice_differences=len(differences),status='static-only')))


if __name__=='__main__':main()
