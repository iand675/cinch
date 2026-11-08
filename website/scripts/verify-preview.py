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
assert (root/'packages/index.html').exists(), 'the package catalogue page must be built'
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
assert catalogue['sample'] is False, 'the catalogue must not advertise sample data'
assert isinstance(catalogue.get('builtAt'), str) and catalogue['builtAt'], 'build time is required'
assert isinstance(catalogue.get('offline'), bool)

if catalogue['offline']:
    assert catalogue['packages'] == [], 'offline builds must not carry package data'
    assert catalogue.get('registry') is None, 'offline builds have no registry source'
    index_html = (root/'packages/index.html').read_text().lower()
    assert 'offline catalogue' in index_html, 'the offline page must say the catalogue is offline'
    assert 'sample data' not in index_html
    print(f'Checked {len(pages)} package/download pages and {links} local links for an offline catalogue.')
elif catalogue.get('fallback'):
    assert isinstance(catalogue.get('warning'), str) and catalogue['warning'], 'a fallback must explain the missing registry'
    index_html = (root/'packages/index.html').read_text().lower()
    assert 'cached catalogue' in index_html, 'the fallback page must say it used a cached catalogue'
    assert 'sample data' not in index_html, 'the fallback page must not advertise sample data'
    packages = catalogue['packages']
    assert isinstance(packages, list)
    for package in packages:
        for release in package['releases']:
            base = root/'packages'/package['name']/release['version']
            assert (base/'index.html').exists(), (package['name'], release['version'], 'fallback package page missing')
    print(f'Checked {len(pages)} package/download pages and {links} local links for a cached catalogue.')
else:
    registry = catalogue.get('registry')
    assert isinstance(registry, str) and registry, 'a live catalogue must name its registry'
    packages = catalogue['packages']
    assert packages, 'a live catalogue must contain at least one public release'
    index_html = (root/'packages/index.html').read_text()
    assert registry in index_html, 'the catalogue index does not name its registry'
    assert 'sample data' not in index_html.lower(), 'the live catalogue index still says sample data'
    assert 'name="robots"' not in index_html, 'the live catalogue index must be indexable'
    expected_pages = 0
    for package in packages:
        assert package['registry'] == registry, (package['name'], 'registry mismatch')
        assert package['releases'], (package['name'], 'no releases')
        for release in package['releases']:
            base = root/'packages'/package['name']/release['version']
            package_page = base/'index.html'
            assert package_page.exists(), (package['name'], release['version'], 'package page missing')
            expected_pages += 1
            package_html = package_page.read_text()
            assert registry in package_html, (package['name'], 'package page does not name its registry')
            assert f'--registry {registry}' in package_html, (package['name'], 'install command must name the registry')
            assert 'sample data' not in package_html.lower()
            assert 'name="robots"' not in package_html, (package['name'], 'live package page must be indexable')
            for module in release['modules']:
                module_page = base/module['name']/'index.html'
                source_page = base/module['name']/'source'/'index.html'
                assert module_page.exists(), (package['name'], release['version'], module['name'], 'module page missing')
                assert source_page.exists(), (package['name'], release['version'], module['name'], 'source page missing')
                expected_pages += 2
                module_html = module_page.read_text()
                assert 'sample api' not in module_html.lower()
                assert 'sample data' not in module_html.lower()
                if not module['declarations']:
                    assert '.cinchdoc' in module_html, (module['name'], 'page must explain generated declaration data')
                assert 'sample listing' not in source_page.read_text().lower()
    built_pages = len(list((root/'packages').rglob('*.html')))
    assert built_pages >= expected_pages + 2, (built_pages, expected_pages, 'missing catalogue routes')

archives = sorted((root/'downloads/files').glob('*.zip'))
# Every release ships every target, so the archive count follows the data.
assert archives, 'no sample download archives were built'
manifests = [json.loads(zipfile.ZipFile(a).read('release.json')) for a in archives]
assert len({(m['channel'], m['target']) for m in manifests}) == len(archives), 'duplicate release target'
assert len({m['target'] for m in manifests}) * len({m['channel'] for m in manifests}) == len(archives), 'a release is missing a target'
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
print(f'Checked {len(pages)} package/download pages, {links} local links, catalogue routes, {len(archives)} ZIPs and checksums, and shared navigation.')
