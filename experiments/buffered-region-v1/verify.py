#!/usr/bin/env python3
"""Verify a declaration-only nominal Region proposal without modifying historical source or checked interfaces."""
from pathlib import Path
import hashlib
import json
import re

HERE=Path(__file__).resolve().parent
ROOT=HERE.parents[1]


def digest(path):
    """Identify original/proposed source and the genuine defining-interface audit bytes independently."""
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main():
    """Reject body/effect or source drift; static verification is explicitly not a compiler or runtime verdict."""
    manifest=json.loads((HERE/'proposal.json').read_text())
    audit_path=HERE/manifest['audit']['frozen_subset']
    assert digest(audit_path)==manifest['audit']['frozen_subset_sha256']
    audit=json.loads(audit_path.read_text())
    assert audit['original_audit_sha256']==manifest['audit']['sha256']
    assert len(manifest['headers'])==10 and len(manifest['modules'])==7
    assert digest(HERE/'embedded-family.cinchpkg')==manifest['qualification_plan']['embedded_family_sha256']
    direct={row['identity']:row for row in audit['candidates']}
    for header in manifest['headers']:
        if header['direct_audit_witness'] is not None:
            assert header['direct_audit_witness']==direct[header['identity']]
            assert header['direct_audit_witness']['recorded_kind']=='Type'
            assert any(':: Region' in use['scheme'] for use in header['direct_audit_witness']['uses'])
    for module,entry in manifest['modules'].items():
        before=ROOT/entry['source'];after=HERE/'sources'/(module+'.cinch')
        assert digest(before)==entry['old_sha256'] and digest(after)==entry['proposal_sha256']
        normalize=lambda text:re.sub(r'\(r :: Region\)','r',text)
        assert normalize(after.read_text())==before.read_text(),module+' has an unintended body/effect edit'
        for header in manifest['headers']:
            if header['identity'].rsplit('.',1)[0]==module:
                assert '  '+header['new'] in after.read_text()
    gate_path=HERE/'source-gate.json'
    if gate_path.exists():
        gate=json.loads(gate_path.read_text())
        assert gate['proposal_sha256']==digest(HERE/'proposal.json')
        assert gate['source_gate_passed'] and gate['compiler_sha256']==manifest['qualification_plan']['compiler_sha256']
        public={row['identity'] for row in manifest['headers'] if row['public']}
        assert {row['identity'] for row in gate['public_contracts']}==public
        assert all(row['parameters'][0]['kind']=='Region' for row in gate['public_contracts'])
        private_path=HERE/'private-gate.json'
        if private_path.exists():
            observed=json.loads(private_path.read_text())
            assert observed['source_gate_sha256']==digest(gate_path)
            assert observed['returncode']==0 and not observed['timeout_without_verdict']
            assert observed['compiler_sha256']==gate['compiler_sha256']
            assert observed['proposal_sha256']==gate['proposal_sha256']
            contract=observed['private_defining_contract']
            assert contract['identity']=='Effect.Stream.WriterChain'
            assert contract['parameters']==[{'implicit':False,'kind':'Region','name':'r'}]
            assert contract['constructors']==['Effect.Stream.WriterEnd','Effect.Stream.WriterLink']
    print('Verified10 annotation-only Region proposals and available separate source/metadata receipts; no runtime or adoption claim')


if __name__=='__main__':main()
