import { existsSync } from 'node:fs';
import { readFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { createMarkdownProcessor } from '@astrojs/markdown-remark';
import rawManifest from '../data/library-sources.json';
import type { LibraryModule, LibraryPackage } from './catalogue';

function findRepoRoot(): string {
  for (const start of [process.cwd(), path.dirname(fileURLToPath(import.meta.url))]) {
    let dir = path.resolve(start);
    for (let i = 0; i < 8; i++) {
      if (existsSync(path.join(dir, 'library')) && existsSync(path.join(dir, 'seed', 'prelude'))) {
        return dir;
      }
      const parent = path.dirname(dir);
      if (parent === dir) break;
      dir = parent;
    }
  }
  return path.resolve(process.cwd(), '..');
}

const REPO_ROOT = findRepoRoot();

export const STDLIB_VERSION = '0.1.0';
export const STDLIB_FAMILIES = [
  'Prelude',
  'Algebra',
  'Category',
  'Relation',
  'Function',
  'Crypto',
  'Data',
  'Text',
  'Effect',
  'Image',
  'Prim',
] as const;

export type StdlibModuleEntry = {
  module: string;
  family: string;
  description: string;
  path: string;
  sha256: string;
  embedded: boolean;
  imports: string[];
  non_embedded_deps: string[];
  types: number;
  signatures: number;
  values: number;
  public_methods: number;
  instances: number;
};

export type StdlibManifest = {
  format: number;
  base_manifest_sha256: string;
  search_entry_count?: number;
  modules: StdlibModuleEntry[];
};

export type StdlibRenderedModule = {
  html: string;
  headings: { depth: number; slug: string; text: string }[];
};

let manifestCache: StdlibManifest | undefined;
let processorPromise: ReturnType<typeof createMarkdownProcessor> | undefined;

export async function loadStdlibManifest(): Promise<StdlibManifest> {
  if (!manifestCache) {
    const parsed = rawManifest as unknown as StdlibManifest;
    manifestCache = {
      ...parsed,
      modules: parsed.modules.map((m) => ({
        ...m,
        family: m.family ?? m.module.split('.')[0] ?? 'Other',
        description: m.description ?? `The ${m.module} module.`,
        imports: m.imports ?? [],
        non_embedded_deps: m.non_embedded_deps ?? [],
        signatures: m.signatures ?? 0,
        instances: m.instances ?? 0,
      })),
    };
  }
  return manifestCache;
}

export async function loadStdlibPackage(): Promise<LibraryPackage> {
  const manifest = await loadStdlibManifest();
  const embeddedCount = manifest.modules.filter((m) => m.embedded).length;
  const sourceCount = manifest.modules.length - embeddedCount;
  const searchCount =
    manifest.search_entry_count ??
    manifest.modules.reduce((acc, m) => acc + m.types + m.signatures + m.values + m.public_methods + m.instances, 0);

  const modules: LibraryModule[] = manifest.modules.map((m) => ({
    name: m.module,
    path: m.path,
    description: m.description,
    imports: m.imports,
    declarations: [],
    embedded: m.embedded,
    family: m.family,
    exportCount: m.types + m.values + m.public_methods,
  }));

  const synopsis = `The Cinch standard library: ${manifest.modules.length} checked modules (${embeddedCount} embedded in the compiler, ${sourceCount} source-distributed) and ${searchCount.toLocaleString('en-US')} indexed declarations.`;

  const release = {
    version: STDLIB_VERSION,
    packageId: `sha256:${manifest.base_manifest_sha256}`,
    yanked: false,
    category: 'Standard Library',
    synopsis,
    description: synopsis,
    license: 'Bundled with Cinch',
    maintainer: 'Cinch Core',
    updated: 'Bundled',
    dependencies: [],
    modules,
    readme: [
      `The Cinch standard library (${manifest.modules.length} modules, ${searchCount.toLocaleString('en-US')} indexed declarations) is generated directly from checked .cinchif interfaces and source documentation comments.`,
      'Embedded modules (66) are compiled into the cinch binary and can be imported immediately. Ship-source modules (222) live under seed/prelude/ and check with the compiler today when copied with their non-embedded closure.',
    ],
  };

  return {
    name: 'stdlib',
    namespace: 'cinch',
    category: 'Standard Library',
    version: STDLIB_VERSION,
    versions: [STDLIB_VERSION],
    yanked: [],
    synopsis,
    description: synopsis,
    license: 'Bundled with Cinch',
    maintainer: 'Cinch Core',
    updated: 'Bundled',
    dependencies: [],
    modules,
    readme: release.readme,
    packageId: release.packageId,
    registry: 'bundled',
    sourceKind: 'stdlib',
    declarationCount: searchCount,
    releases: [release],
  };
}

function getMarkdownProcessor(): ReturnType<typeof createMarkdownProcessor> {
  processorPromise ??= createMarkdownProcessor();
  return processorPromise;
}

/**
 * Rewrite relative Markdown links inside generated `library/<Module>.md` files
 * to canonical `/packages/stdlib/` and `/reference/` routes, demote top-level
 * `#` headings to `##` (the page header owns `<h1>`), and strip the top title
 * / summary / `.doc-mod-bar` rendered natively by the package template.
 */
function preprocessModuleMarkdown(raw: string, knownModules: Set<string>): string {
  const lines = raw.split('\n');
  let start = 0;

  // Skip leading `# ModuleName`
  while (start < lines.length && lines[start]!.trim() === '') start += 1;
  if (lines[start]?.startsWith('# ')) start += 1;

  // Skip the one-line summary and `.doc-mod-bar` block (rendered in the package header)
  while (start < lines.length && lines[start]!.trim() === '') start += 1;
  const modBarIdx = lines.findIndex((line, idx) => idx >= start && idx < start + 12 && line.startsWith('<div class="doc-mod-bar">'));
  if (modBarIdx >= 0) {
    start = modBarIdx + 1;
    while (start < lines.length && lines[start]!.trim() === '') start += 1;
  }

  const out: string[] = [];
  let fence: string | null = null;

  for (const line of lines.slice(start)) {
    const fenceMatch = /^(\s*)(```+|~~~+)([\w+-]*)(.*)$/.exec(line);
    if (fenceMatch) {
      const marker = fenceMatch[2]![0]!;
      if (fence === null) fence = marker;
      else if (fence === marker) fence = null;
      out.push(line);
      continue;
    }
    if (fence !== null) {
      out.push(line);
      continue;
    }

    const demoted = line.replace(/^(#{1,5})(\s+)/, '#$1$2');
    out.push(
      demoted.replace(
        /(\]\()([^)\s]+)((?:\s+"[^"]*")?\))/g,
        (whole, open: string, target: string, close: string) => {
          if (
            /^[a-z][a-z0-9+.-]*:/i.test(target) ||
            target.startsWith('/') ||
            target.startsWith('#')
          ) {
            return whole;
          }
          const [pathPart, ...rest] = target.split('#');
          const hash = rest.length > 0 ? `#${rest.join('#')}` : '';
          if (!pathPart) return whole;

          if (pathPart === '00-index.md') {
            return `${open}/packages/stdlib/${hash}${close}`;
          }
          if (pathPart.endsWith('.md') && !pathPart.includes('/')) {
            const mod = pathPart.slice(0, -3);
            if (knownModules.has(mod)) {
              return `${open}/packages/stdlib/${mod}/${hash}${close}`;
            }
          }
          if (pathPart.startsWith('../reference/') && pathPart.endsWith('.md')) {
            return `${open}/reference/${pathPart.slice('../reference/'.length, -3)}/${hash}${close}`;
          }
          if (pathPart.startsWith('../learn/') && pathPart.endsWith('.md')) {
            return `${open}/learn/${pathPart.slice('../learn/'.length, -3)}/${hash}${close}`;
          }
          return whole;
        },
      ),
    );
  }

  return out.join('\n');
}

export async function renderStdlibModule(moduleName: string): Promise<StdlibRenderedModule> {
  const manifest = await loadStdlibManifest();
  const knownModules = new Set(manifest.modules.map((m) => m.module));
  const raw = await readFile(path.join(REPO_ROOT, 'library', `${moduleName}.md`), 'utf8');
  const markdown = preprocessModuleMarkdown(raw, knownModules);
  const processor = await getMarkdownProcessor();
  const rendered = await processor.render(markdown);
  const headings = (rendered.metadata.headings ?? []).filter((h) => h.depth === 2);
  return {
    html: rendered.code,
    headings,
  };
}

export async function readStdlibSource(relPath: string): Promise<string> {
  return readFile(path.join(REPO_ROOT, relPath), 'utf8');
}
