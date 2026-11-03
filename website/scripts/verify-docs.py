"""Audit the built tutorial, compiler guide, and real library reference."""
from pathlib import Path
from html.parser import HTMLParser
from urllib.parse import unquote, urljoin, urlsplit
import json

SITE = Path(__file__).resolve().parents[1]
DIST = SITE / 'dist'
class Page(HTMLParser):
    def __init__(self, path):
        super().__init__()
        self.ids=set();self.links=[];self.duplicates=[];self.text=[]
        self.feed(path.read_text())
    def handle_starttag(self, tag, attrs):
        attributes=dict(attrs)
        if 'id' in attributes:
            value=attributes['id']
            if value in self.ids:self.duplicates.append(value)
            self.ids.add(value)
        if tag=='a' and attributes.get('href'):self.links.append(attributes['href'])
    def handle_data(self,text):self.text.append(text)

cache={}
def page(path):
    if path not in cache:cache[path]=Page(path)
    return cache[path]

def main():
    files=[DIST/'documentation/index.html',DIST/'index.html']
    for family in ['learn','reference','library']:files.extend(sorted((DIST/family).rglob('*.html')))
    assert len(files)>130,'Expected tutorial, guide, and all real module pages'
    errors=[];links=0
    for file in files:
        parsed=page(file)
        if parsed.duplicates:errors.append((str(file.relative_to(DIST)),'duplicate IDs',parsed.duplicates))
        base='/'+str(file.relative_to(DIST)).removesuffix('index.html')
        for href in parsed.links:
            url=urlsplit(urljoin(base,href))
            if url.scheme or url.netloc:continue
            target=DIST/unquote(url.path).lstrip('/')
            if not target.suffix or target.is_dir():target=target/'index.html'
            if not target.exists():errors.append((str(file.relative_to(DIST)),href,'missing target'));continue
            if url.fragment and target.suffix=='.html' and unquote(url.fragment) not in page(target).ids:
                errors.append((str(file.relative_to(DIST)),href,'missing anchor'))
            links+=1
    for legacy in ['overview','tutorials/01-values-and-dependent-types','specifications/core-language-specification']:
        target=DIST/legacy/'index.html'
        assert target.exists(),f'Preserve existing route: {legacy}'
        assert 'original design archive' in ''.join(page(target).text),f'Archive status missing: {legacy}'
    status=''.join(page(DIST/'reference/16-support-and-roadmap/index.html').text)
    for label in ['Coming soon','Partial','API only','Rust compiler']:assert label in status,label
    assert 'support-tag--planned' in (DIST/'reference/16-support-and-roadmap/index.html').read_text()
    for file in [DIST/'learn/01-first-program/index.html',DIST/'reference/15-ffi-layout-and-attributes/index.html']:
        assert 'data-language="cinch"' in file.read_text(),'Cinch examples must render as highlighted code'
    sources=json.loads((SITE/'src/data/library-sources.json').read_text())['modules']
    for entry in sources:
        assert (DIST/'library'/entry['module']/'index.html').exists(),entry['module']
        assert (DIST/'reference-sources'/(entry['module']+'.cinch')).exists(),entry['module']
    if errors:
        for error in errors[:40]:print(error)
        raise AssertionError(f'{len(errors)} documentation link/anchor failures')
    print(f'Checked {len(files)} reader/landing pages, {links} local links, {len(sources)} module pages/downloads, support labels, code rendering, and preserved archive routes')

if __name__=='__main__':main()
