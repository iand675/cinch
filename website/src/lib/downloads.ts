import {project} from './project.ts';

export const targets = [
  {id: 'macos-arm64', system: 'aarch64-darwin', os: 'macos', arch: 'arm64', name: 'macOS', architecture: 'Apple Silicon'},
  {id: 'linux-x64', system: 'x86_64-linux', os: 'linux', arch: 'x64', name: 'Linux', architecture: 'x86-64'},
  {id: 'linux-arm64', system: 'aarch64-linux', os: 'linux', arch: 'arm64', name: 'Linux', architecture: 'ARM64'},
] as const;

export interface NightlyArtifact {
  target: typeof targets[number];
  filename: string;
  url: string;
  checksumUrl: string;
  sha256: string;
  size: number;
}
export interface NightlyRelease {
  tag: string;
  url: string;
  publishedAt: string;
  revision: string;
  artifacts: NightlyArtifact[];
}
interface ReleaseAsset { name: string; browser_download_url: string; digest: string | null; size: number; }
interface GitHubRelease {
  tag_name: string;
  html_url: string;
  published_at: string;
  target_commitish: string;
  draft: boolean;
  prerelease: boolean;
  assets: ReleaseAsset[];
}

// Keep release metadata and download links confined to this repository.
function releaseUrl(value: string, prefix: string): boolean {
  return value.startsWith(`${project.repository}/${prefix}/`);
}

export function selectNightly(releases: GitHubRelease[]): NightlyRelease | null {
  const nightlies = releases.filter(release => !release.draft && release.prerelease
    && /^nightly-\d{8}-[a-f0-9]{12}-\d+$/.test(release.tag_name)
    && /^[a-f0-9]{40}$/.test(release.target_commitish)
    && Number.isFinite(Date.parse(release.published_at))
    && releaseUrl(release.html_url, 'releases/tag'));
  nightlies.sort((left, right) => Date.parse(right.published_at) - Date.parse(left.published_at));
  for (const release of nightlies) {
    const artifacts: NightlyArtifact[] = [];
    for (const target of targets) {
      const filename = `cinch-${target.system}.nar.gz`;
      const asset = release.assets.find(value => value.name === filename);
      const checksum = release.assets.find(value => value.name === `${filename}.sha256`);
      if (!asset || !checksum || !/^sha256:[a-f0-9]{64}$/.test(asset.digest ?? '')
        || !releaseUrl(asset.browser_download_url, `releases/download/${release.tag_name}`)
        || !releaseUrl(checksum.browser_download_url, `releases/download/${release.tag_name}`)
        || !Number.isSafeInteger(asset.size) || asset.size <= 0) break;
      artifacts.push({target, filename, url: asset.browser_download_url,
        checksumUrl: checksum.browser_download_url, sha256: asset.digest!.slice(7), size: asset.size});
    }
    if (artifacts.length === targets.length) return {
      tag: release.tag_name, url: release.html_url, publishedAt: release.published_at,
      revision: release.target_commitish, artifacts,
    };
  }
  return null;
}

export async function loadNightly(): Promise<NightlyRelease | null> {
  const response = await fetch('https://api.github.com/repos/iand675/cinch/releases?per_page=30', {
    headers: {Accept: 'application/vnd.github+json'},
    signal: AbortSignal.timeout(10000),
  });
  if (!response.ok) throw new Error(`GitHub releases returned ${response.status}`);
  const releases: unknown = await response.json();
  if (!Array.isArray(releases)) throw new Error('GitHub returned an invalid release list');
  return selectNightly(releases);
}
