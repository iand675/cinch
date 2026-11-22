import assert from 'node:assert/strict';
import {selectNightly, targets} from '../src/lib/downloads.ts';

const repo = 'https://github.com/iand675/cinch';
function nightly(day: string, suffix: string) {
  const tag = `nightly-202611${day}-${'a'.repeat(12)}-${suffix}`;
  return {tag_name: tag, target_commitish: 'a'.repeat(40), html_url: `${repo}/releases/tag/${tag}`,
    published_at: `2026-11-${day}T03:00:00Z`, draft: false, prerelease: true,
    assets: targets.flatMap(target => {
      const name = `cinch-${target.system}`;
      return [name, `${name}.sha256`].map(filename => ({name: filename,
        browser_download_url: `${repo}/releases/download/${tag}/${filename}`,
        digest: `sha256:${'b'.repeat(64)}`, size: 1234}));
    })};
}
const older = nightly('21', '1');
const newer = nightly('22', '2');
assert.equal(selectNightly([]), null);
assert.equal(selectNightly([older, newer])?.tag, newer.tag_name);
assert.equal(selectNightly([{...newer, draft: true}, older])?.tag, older.tag_name);
assert.equal(selectNightly([{...newer, assets: newer.assets.slice(1)}, older])?.tag, older.tag_name);
assert.equal(selectNightly([{...newer, assets: newer.assets.map(asset => ({...asset, digest: null}))}, older])?.tag, older.tag_name);
assert.equal(selectNightly([{...newer, assets: newer.assets.map(asset => ({...asset, browser_download_url: 'https://example.com/file'}))}, older])?.tag, older.tag_name);
assert.equal(selectNightly([{...newer, assets: newer.assets.map(asset => ({...asset, size: 0}))}, older])?.tag, older.tag_name);
assert.equal(selectNightly([{...newer, target_commitish: 'main'}, older])?.tag, older.tag_name);
assert.equal(selectNightly([{...newer, prerelease: false}]), null);
const selected = selectNightly([newer])!;
assert.equal(selected.artifacts.length, 3);
for (const artifact of selected.artifacts) {
  assert.equal(artifact.sha256, 'b'.repeat(64));
  assert.equal(artifact.checksumUrl, `${artifact.url}.sha256`);
  assert.ok(artifact.filename.includes(artifact.target.system));
}
console.log('Checked nightly selection, three platforms, incomplete releases, digests, URLs and revision identity.');
