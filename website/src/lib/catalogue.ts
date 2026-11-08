import { Buffer } from 'node:buffer';
import { readFile } from 'node:fs/promises';

/**
 * Build-time catalogue loader.
 *
 * Astro builds this site statically, so every registry request happens once
 * during `npm run build`. Nothing in this module runs in a browser.
 *
 * The loader reads public releases from the v0 registry API, groups them by
 * package name, and fetches each release manifest. Declaration signatures are
 * not part of a registry release, so modules start with an empty declaration
 * list. Pages render the facts that exist and say where generated `.cinchdoc`
 * data would come from.
 */

export type DeclarationKind = 'type' | 'function' | 'class';

export type Declaration = {
  name: string;
  kind: DeclarationKind;
  signature: string;
  description: string;
  example?: string;
  constructors?: { name: string; signature: string; description: string }[];
  since?: string;
};

export type Dependency = {
  name: string;
  range?: string;
  source: string;
  detail?: string;
};

export type LibraryModule = {
  name: string;
  path: string;
  description: string;
  imports: string[];
  declarations: Declaration[];
  source?: string[];
};

export type LibraryRelease = {
  version: string;
  packageId: string;
  yanked: boolean;
  category: string;
  synopsis: string;
  description: string;
  license: string;
  maintainer: string;
  updated: string;
  dependencies: Dependency[];
  modules: LibraryModule[];
  readme: string[];
};

export type LibraryPackage = {
  name: string;
  namespace: string;
  category: string;
  version: string;
  versions: string[];
  yanked: string[];
  synopsis: string;
  description: string;
  license: string;
  maintainer: string;
  updated: string;
  dependencies: Dependency[];
  modules: LibraryModule[];
  readme: string[];
  packageId: string;
  registry: string;
  sourceKind: 'registry';
  releases: LibraryRelease[];
};

export type Catalogue = {
  registry: string | null;
  builtAt: string;
  offline: boolean;
  fallback: boolean;
  warning?: string;
  snapshotBuiltAt?: string;
  packages: LibraryPackage[];
};

export const CATALOGUE_REGISTRY_ENV = 'CINCH_CATALOGUE_REGISTRY';
export const CATALOGUE_OFFLINE_ENV = 'CINCH_CATALOGUE_OFFLINE';
export const DEFAULT_CATALOGUE_REGISTRY = 'http://127.0.0.1:8787';

type IndexEntry = {
  name: string;
  version: string;
  packageId?: string;
  yanked?: boolean;
  blocked?: boolean;
  createdAt?: string;
};

type Manifest = {
  description?: unknown;
  category?: unknown;
  license?: unknown;
  maintainers?: unknown;
  modules?: unknown;
  dependencies?: unknown;
};

type ReleaseMetadata = {
  manifest?: Manifest;
  readme?: unknown;
  packageId?: unknown;
  namespace?: unknown;
  createdAt?: unknown;
};

type Bundle = { files?: Record<string, string> };

function readEnv(name: string): string | undefined {
  if (typeof process !== 'undefined' && process.env) {
    const value = process.env[name];
    if (value !== undefined) return value;
  }
  const build = (import.meta.env as Record<string, string | undefined>)[name];
  return build;
}

export function catalogueOffline(): boolean {
  return readEnv(CATALOGUE_OFFLINE_ENV) === '1';
}

export function catalogueRegistry(): string {
  const configured = readEnv(CATALOGUE_REGISTRY_ENV)?.trim();
  return (configured && configured.length > 0 ? configured : DEFAULT_CATALOGUE_REGISTRY).replace(/\/+$/, '');
}

function message(error: unknown): string {
  return error instanceof Error ? error.message : String(error);
}

function failure(registry: string, reason: string): Error {
  return new Error(
    `Cinch catalogue: ${reason}. ` +
      `Set ${CATALOGUE_REGISTRY_ENV} to a reachable registry (currently ${registry || 'unset'}), ` +
      `or set ${CATALOGUE_OFFLINE_ENV}=1 to build the offline catalogue page.`,
  );
}

async function getJson<T>(url: string, what: string): Promise<T> {
  let response: Response;
  try {
    response = await fetch(url, {
      headers: { accept: 'application/json' },
      signal: AbortSignal.timeout(20000),
    });
  } catch (error) {
    throw new Error(`${what} at ${url} failed: ${message(error)}`);
  }
  if (!response.ok) throw new Error(`${what} at ${url} returned HTTP ${response.status}`);
  try {
    return (await response.json()) as T;
  } catch (error) {
    throw new Error(`${what} at ${url} did not return JSON: ${message(error)}`);
  }
}

async function mapLimit<T, R>(items: T[], limit: number, run: (item: T) => Promise<R>): Promise<R[]> {
  const results = new Array<R>(items.length);
  let next = 0;
  const workers = Array.from({ length: Math.min(limit, items.length) }, async () => {
    while (true) {
      const index = next++;
      if (index >= items.length) return;
      results[index] = await run(items[index]);
    }
  });
  await Promise.all(workers);
  return results;
}

function text(value: unknown): string | undefined {
  return typeof value === 'string' && value.trim().length > 0 ? value.trim() : undefined;
}

function indexEntry(value: unknown): value is IndexEntry {
  if (!value || typeof value !== 'object') return false;
  const item = value as Record<string, unknown>;
  return typeof item.name === 'string' && item.name.includes('/') && typeof item.version === 'string';
}

function decodeBase64(value: string): string {
  return Buffer.from(value, 'base64').toString('utf8');
}

function normalizeDependencies(raw: unknown): Dependency[] {
  if (!Array.isArray(raw)) return [];
  const dependencies: Dependency[] = [];
  for (const value of raw) {
    if (typeof value === 'string') {
      dependencies.push({ name: value, source: 'registry' });
      continue;
    }
    if (!value || typeof value !== 'object') continue;
    const item = value as Record<string, unknown>;
    const name = text(item.name);
    if (!name) continue;
    const source =
      typeof item.source === 'string'
        ? item.source
        : typeof item.range === 'string'
          ? 'registry'
          : typeof item.path === 'string'
            ? 'path'
            : typeof item.url === 'string'
              ? 'git'
              : 'registry';
    const detail =
      typeof item.path === 'string'
        ? item.path
        : typeof item.url === 'string'
          ? [item.url, text(item.rev)].filter(Boolean).join(' @ ')
          : undefined;
    dependencies.push({ name, range: text(item.range), source, detail });
  }
  return dependencies;
}

function normalizeModules(raw: unknown, files: Record<string, string>): LibraryModule[] {
  if (!Array.isArray(raw)) return [];
  const modules: LibraryModule[] = [];
  for (const value of raw) {
    if (!value || typeof value !== 'object') continue;
    const item = value as Record<string, unknown>;
    const name = text(item.name);
    if (!name) continue;
    const path = text(item.path) ?? '';
    const encoded = path.length > 0 ? files[path] : undefined;
    modules.push({
      name,
      path,
      description: text(item.description) ?? '',
      imports: [],
      declarations: [],
      source:
        typeof encoded === 'string'
          ? decodeBase64(encoded).replace(/\r\n/g, '\n').split('\n')
          : undefined,
    });
  }
  return modules;
}

function readmeParagraphs(value: unknown): string[] {
  const raw = text(value);
  if (!raw) return ['This release does not include a README.'];
  const paragraphs = raw
    .split(/\n\s*\n/)
    .map((paragraph) => paragraph.trim())
    .filter(Boolean);
  return paragraphs.length > 0 ? paragraphs : [raw];
}

function maintainerOf(manifest: Manifest, namespace: string): string {
  const value = manifest.maintainers;
  if (typeof value === 'string' && value.trim().length > 0) return value.trim();
  if (Array.isArray(value)) {
    const names = value.filter((item): item is string => typeof item === 'string').map((item) => item.trim()).filter(Boolean);
    if (names.length > 0) return names.join(', ');
  }
  return namespace;
}

async function loadRelease(
  registry: string,
  name: string,
  entry: IndexEntry,
  builtAt: string,
): Promise<LibraryRelease> {
  const url = `${registry}/api/v1/packages/${encodeURIComponent(name)}/${encodeURIComponent(entry.version)}`;
  let metadata: ReleaseMetadata;
  try {
    metadata = await getJson<ReleaseMetadata>(url, `metadata for ${name}@${entry.version}`);
  } catch (error) {
    throw failure(registry, `could not read metadata for ${name}@${entry.version}: ${message(error)}`);
  }

  const manifest = metadata.manifest ?? {};
  const digest = String(metadata.packageId ?? entry.packageId ?? '').replace(/^sha256:/, '');
  let files: Record<string, string> = {};
  if (/^[0-9a-f]{64}$/.test(digest)) {
    try {
      const bundle = await getJson<Bundle>(`${registry}/api/v1/download/${digest}`, `bundle for ${name}@${entry.version}`);
      files = bundle.files ?? {};
    } catch {
      files = {};
    }
  }

  const namespace = text(metadata.namespace) ?? name.split('/')[0] ?? name;
  const description = text(manifest.description);
  const readme = readmeParagraphs(metadata.readme);
  const synopsis = description ?? readme[0].split('\n')[0].trim();

  return {
    version: entry.version,
    packageId: String(metadata.packageId ?? entry.packageId ?? ''),
    yanked: entry.yanked === true,
    category: text(manifest.category) ?? 'Registry',
    synopsis,
    description: description ?? synopsis,
    license: text(manifest.license) ?? 'Not declared',
    maintainer: maintainerOf(manifest, namespace),
    updated: text(metadata.createdAt) ?? text(entry.createdAt) ?? builtAt,
    dependencies: normalizeDependencies(manifest.dependencies),
    modules: normalizeModules(manifest.modules, files),
    readme,
  };
}

async function loadPackage(
  registry: string,
  name: string,
  entries: IndexEntry[],
  builtAt: string,
): Promise<LibraryPackage> {
  const ordered = [...entries].sort((a, b) => compareVersions(b.version, a.version) || b.version.localeCompare(a.version));
  const releases = await mapLimit(ordered, 4, (entry) => loadRelease(registry, name, entry, builtAt));
  const latest = releases.find((release) => !release.yanked) ?? releases[0];
  return {
    name,
    namespace: name.split('/')[0] ?? name,
    category: latest.category,
    version: latest.version,
    versions: releases.map((release) => release.version),
    yanked: releases.filter((release) => release.yanked).map((release) => release.version),
    synopsis: latest.synopsis,
    description: latest.description,
    license: latest.license,
    maintainer: latest.maintainer,
    updated: latest.updated,
    dependencies: latest.dependencies,
    modules: latest.modules,
    readme: latest.readme,
    packageId: latest.packageId,
    registry,
    sourceKind: 'registry',
    releases,
  };
}

let cache: Promise<Catalogue> | undefined;

export function loadCatalogue(): Promise<Catalogue> {
  cache ??= buildCatalogue();
  return cache;
}

const SNAPSHOT_URL = new URL('../data/catalogue-snapshot.json', import.meta.url);

async function loadSnapshot(registry: string, reason: string): Promise<Catalogue> {
  const builtAt = new Date().toISOString();
  try {
    const text = await readFile(SNAPSHOT_URL, 'utf8');
    const snapshot = JSON.parse(text) as Partial<Catalogue>;
    if (
      !snapshot ||
      !Array.isArray(snapshot.packages) ||
      typeof snapshot.builtAt !== 'string'
    ) {
      throw new Error('snapshot is missing its package list');
    }
    return {
      registry: snapshot.registry ?? registry,
      builtAt: snapshot.builtAt,
      offline: false,
      fallback: true,
      warning: `Live registry unavailable: ${reason}. Showing the last synced catalogue from ${snapshot.builtAt}.`,
      snapshotBuiltAt: snapshot.builtAt,
      packages: snapshot.packages,
    };
  } catch (error) {
    return {
      registry,
      builtAt,
      offline: false,
      fallback: true,
      warning: `Live registry unavailable (${reason}) and no cached catalogue is present. Run catalogue:snapshot after a successful live build. ${message(error)}`,
      packages: [],
    };
  }
}

async function buildCatalogue(): Promise<Catalogue> {
  const builtAt = new Date().toISOString();
  if (catalogueOffline()) {
    return { registry: null, builtAt, offline: true, fallback: false, packages: [] };
  }

  const registry = catalogueRegistry();
  try {
    let index: { packages?: unknown };
    try {
      index = await getJson<{ packages?: unknown }>(`${registry}/api/v1/index`, 'public index');
    } catch (error) {
      throw failure(registry, message(error));
    }

    const entries = (Array.isArray(index.packages) ? index.packages : []).filter(indexEntry);
    const groups = new Map<string, IndexEntry[]>();
    for (const entry of entries) {
      if (entry.blocked === true) continue;
      const group = groups.get(entry.name) ?? [];
      group.push(entry);
      groups.set(entry.name, group);
    }

    if (groups.size === 0) {
      throw failure(registry, `the registry at ${registry} returned no public releases`);
    }

    const packages = await mapLimit([...groups.entries()], 8, ([name, group]) =>
      loadPackage(registry, name, group, builtAt),
    );
    packages.sort((a, b) => a.name.localeCompare(b.name));
    return { registry, builtAt, offline: false, fallback: false, packages };
  } catch (error) {
    return loadSnapshot(registry, message(error));
  }
}

export type PublicModule = LibraryModule;
export type PublicRelease = Omit<LibraryRelease, 'modules'> & { modules: PublicModule[] };
export type PublicPackage = Omit<LibraryPackage, 'modules' | 'releases'> & {
  modules: PublicModule[];
  releases: PublicRelease[];
};

function publicModule(module: LibraryModule): PublicModule {
  return {
    name: module.name,
    path: module.path,
    description: module.description,
    imports: module.imports,
    declarations: module.declarations,
    source: module.source,
  };
}

export function publicCatalogue(catalogue: Catalogue): {
  schema: number;
  sample: false;
  offline: boolean;
  fallback: boolean;
  warning: string | null;
  snapshotBuiltAt: string | null;
  registry: string | null;
  builtAt: string;
  packages: PublicPackage[];
} {
  return {
    schema: 2,
    sample: false,
    offline: catalogue.offline,
    fallback: catalogue.fallback,
    warning: catalogue.warning ?? null,
    snapshotBuiltAt: catalogue.snapshotBuiltAt ?? null,
    registry: catalogue.registry,
    builtAt: catalogue.builtAt,
    packages: catalogue.packages.map((pkg) => ({
      name: pkg.name,
      namespace: pkg.namespace,
      category: pkg.category,
      version: pkg.version,
      versions: pkg.versions,
      yanked: pkg.yanked,
      synopsis: pkg.synopsis,
      description: pkg.description,
      license: pkg.license,
      maintainer: pkg.maintainer,
      updated: pkg.updated,
      dependencies: pkg.dependencies,
      modules: pkg.modules.map(publicModule),
      readme: pkg.readme,
      packageId: pkg.packageId,
      registry: pkg.registry,
      sourceKind: pkg.sourceKind,
      releases: pkg.releases.map((release) => ({
        version: release.version,
        packageId: release.packageId,
        yanked: release.yanked,
        category: release.category,
        synopsis: release.synopsis,
        description: release.description,
        license: release.license,
        maintainer: release.maintainer,
        updated: release.updated,
        dependencies: release.dependencies,
        modules: release.modules.map(publicModule),
        readme: release.readme,
      })),
    })),
  };
}

export const packageUrl = (p: LibraryPackage, version = p.version) => `/packages/${p.name}/${version}/`;
export const moduleUrl = (p: LibraryPackage, m: LibraryModule, version = p.version) =>
  `${packageUrl(p, version)}${m.name}/`;
export const symbolId = (name: string) => `decl-${encodeURIComponent(name).replace(/%/g, '_')}`;
export const compareVersions = (a: string, b: string) => {
  const x = a.split('.').map(Number);
  const y = b.split('.').map(Number);
  for (let i = 0; i < 3; i++) {
    const difference = (x[i] ?? 0) - (y[i] ?? 0);
    if (difference) return difference;
  }
  return 0;
};
export const declarationsFor = (m: LibraryModule, version: string) =>
  m.declarations.filter((declaration) => !declaration.since || compareVersions(version, declaration.since) >= 0);
export const symbolUrl = (p: LibraryPackage, m: LibraryModule, d: Declaration, version = p.version) =>
  `${moduleUrl(p, m, version)}#${symbolId(d.name)}`;
export const dependencyLabel = (dependency: Dependency) => {
  if (dependency.range) return dependency.range;
  if (dependency.source === 'path' && dependency.detail) return dependency.detail;
  if (dependency.source === 'git' && dependency.detail) return dependency.detail;
  return dependency.source;
};
export function resolveType(name: string, p: LibraryPackage, _m: LibraryModule, version: string) {
  const modules = p.releases.find((release) => release.version === version)?.modules ?? p.modules;
  for (const module of modules) {
    for (const declaration of declarationsFor(module, version)) {
      if (declaration.name === name && declaration.kind !== 'function') return symbolUrl(p, module, declaration, version);
    }
  }
  return undefined;
}
