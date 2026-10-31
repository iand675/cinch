"""Check the built catalogue's navigable surface and actual downloadable archives."""
from pathlib import Path
from html.parser import HTMLParser
from urllib.parse import urlsplit, unquote
import hashlib
import json
import zipfile

root = Path(__file__).resolve().parents[1] / 'dist'
class Page(HTMLParser):
    def __init__(self, path):
        super().__init__()
        self.ids = set()
        self.links = []
        self.duplicates = []
        self.feed(path.read_text())
    def handle_starttag(self, tag, attrs):
        values = dict(attrs)
        if 'id' in values:
            if values['id'] in self.ids: self.duplicates.append(values['id'])
            self.ids.add(values['id'])
        if tag == 'a' and values.get('href'): self.links.append(values['href'])

cache = {}
def page(path):
    if path not in cache: cache[path] = Page(path)
    return cache[path]
def resolve(path):
    file = root / unquote(path).lstrip('/')
    return file / 'index.html' if not file.suffix or file.is_dir() else file

pages = sorted((root / 'packages').rglob('*.html')) + [root/'downloads/index.html']
assert len(pages) > 70, 'Package, version, API and declaration-source pages must all be built'
links = 0
for file in pages:
    parsed = page(file)
    assert not parsed.duplicates, (file, parsed.duplicates)
    for href in parsed.links:
        url = urlsplit(href)
        if url.scheme or url.netloc: continue
        if not url.path: target = file
        elif url.path.startswith('/'): target = resolve(url.path)
        else: target = resolve('/' + str(file.parent.relative_to(root) / url.path))
        assert target.exists(), (file.relative_to(root), href, 'missing destination')
        if url.fragment and target.suffix == '.html':
            assert unquote(url.fragment) in page(target).ids, (file.relative_to(root), href, 'missing anchor')
        links += 1

catalogue = json.loads((root/'packages/index.json').read_text())
assert catalogue['sample'] is True
assert len(catalogue['packages']) == 8
for package in catalogue['packages']:
    for version in package['versions']:
        assert (root/f"packages/{package['name']}/{version}/index.html").exists()
        for module in package['modules']:
            assert (root/f"packages/{package['name']}/{version}/{module['name']}/index.html").exists()
            assert (root/f"packages/{package['name']}/{version}/{module['name']}/source/index.html").exists()
assert 'decl-with-timeout' in page(root/'packages/cinch-http/0.4.0/Net.Http.Client/index.html').ids
assert 'decl-with-timeout' not in page(root/'packages/cinch-http/0.3.0/Net.Http.Client/index.html').ids

archives = sorted((root/'downloads/files').glob('*.zip'))
assert len(archives) == 8
for archive in archives:
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    assert archive.with_name(archive.name+'.sha256').read_text().split()[0] == digest
    with zipfile.ZipFile(archive) as bundle:
        assert bundle.testzip() is None
        assert set(bundle.namelist()) == {'README.txt','release.json'}
        metadata = json.loads(bundle.read('release.json'))
        assert metadata['sample'] and metadata['containsCompiler'] is False
        assert metadata['target'] in archive.name and metadata['version'] in archive.name
        assert b'SAMPLE archive' in bundle.read('README.txt')

repository = 'https://github.com/iand675/cinch'
for file in [root/'index.html',root/'downloads/index.html',root/'packages/index.html']:
    assert repository in page(file).links
    assert '/downloads/' in page(file).links and '/packages/' in page(file).links
for file in (root/'tutorials').rglob('*.html'):
    assert repository in page(file).links and '/downloads/' in page(file).links and '/packages/' in page(file).links
print(f'Checked {len(pages)} package/download pages, {links} local links, versioned API anchors, {len(archives)} ZIPs and checksums, and shared navigation.')
